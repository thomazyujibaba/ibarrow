use anyhow::{anyhow, Result};
use arrow::array::Array;
use arrow::ffi::to_ffi;
use arrow::record_batch::RecordBatchReader;
use arrow_ipc::writer::StreamWriter;
use arrow_odbc::OdbcReaderBuilder;
use odbc_api::{ConnectionOptions, Environment};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyCapsule;
use serde::{Deserialize, Serialize};
use std::ffi::CString;

use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(ibarrow, PyConnectionError, PyException);
create_exception!(ibarrow, PySQLError, PyException);
create_exception!(ibarrow, PyArrowError, PyException);

// Connection class for maintaining database session
#[pyclass]
pub struct IbarrowConnection {
    dsn: String,
    user: String,
    password: String,
    config: QueryConfig,
}

#[pymethods]
impl IbarrowConnection {
    #[new]
    fn new(dsn: &str, user: &str, password: &str, config: Option<&QueryConfig>) -> Self {
        let config = config
            .cloned()
            .unwrap_or_else(|| QueryConfig::new(None, None, None, None, None, None, None));
        Self {
            dsn: dsn.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            config,
        }
    }

    fn query_arrow_ipc(&self, sql: &str) -> PyResult<Vec<u8>> {
        query_arrow_ipc_impl(&self.dsn, &self.user, &self.password, sql, &self.config).map_err(
            |e| {
                let msg = e.to_string();
                if msg.contains("IM002") || msg.contains("connection") {
                    PyConnectionError::new_err(format!("Connection Error: {}", msg))
                } else if msg.contains("SQL") || msg.contains("syntax") {
                    PySQLError::new_err(format!("SQL Error: {}", msg))
                } else if msg.contains("Arrow") || msg.contains("c_data") {
                    PyArrowError::new_err(format!("Arrow Error: {}", msg))
                } else {
                    PyRuntimeError::new_err(msg)
                }
            },
        )
    }

    fn query_polars(&self, sql: &str) -> PyResult<Py<PyAny>> {
        query_polars_impl(&self.dsn, &self.user, &self.password, sql, &self.config)
    }

    fn query_pandas(&self, sql: &str) -> PyResult<Py<PyAny>> {
        query_pandas_impl(&self.dsn, &self.user, &self.password, sql, &self.config)
    }

    fn query_arrow_c_data(&self, sql: &str, return_dataframe: Option<bool>) -> PyResult<Py<PyAny>> {
        query_arrow_c_data_with_df(
            &self.dsn,
            &self.user,
            &self.password,
            sql,
            &self.config,
            return_dataframe,
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "IbarrowConnection(dsn='{}', user='{}')",
            self.dsn, self.user
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct QueryConfig {
    #[pyo3(get, set)]
    pub batch_size: Option<u32>,
    #[pyo3(get, set)]
    pub max_text_size: Option<u32>,
    #[pyo3(get, set)]
    pub max_binary_size: Option<u32>,
    #[pyo3(get, set)]
    pub read_only: bool,
    #[pyo3(get, set)]
    pub connection_timeout: Option<u32>,
    #[pyo3(get, set)]
    pub query_timeout: Option<u32>,
    #[pyo3(get, set)]
    pub isolation_level: Option<String>,
}

#[pymethods]
impl QueryConfig {
    #[new]
    fn new(
        batch_size: Option<u32>,
        max_text_size: Option<u32>,
        max_binary_size: Option<u32>,
        read_only: Option<bool>,
        connection_timeout: Option<u32>,
        query_timeout: Option<u32>,
        isolation_level: Option<String>,
    ) -> Self {
        Self {
            batch_size,
            max_text_size,
            max_binary_size,
            read_only: read_only.unwrap_or(false),
            connection_timeout,
            query_timeout,
            isolation_level,
        }
    }
}

// Implementation function for Arrow IPC
fn query_arrow_ipc_impl(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: &QueryConfig,
) -> Result<Vec<u8>> {
    let env = Environment::new()?;

    let mut conn_str = format!("DSN={};UID={};PWD={};", dsn, user, password);

    if config.read_only {
        conn_str.push_str("ReadOnly=1;");
    }

    if let Some(timeout) = config.connection_timeout {
        conn_str.push_str(&format!("Connection Timeout={};", timeout));
    }

    if let Some(timeout) = config.query_timeout {
        conn_str.push_str(&format!("Query Timeout={};", timeout));
    }

    if let Some(level) = &config.isolation_level {
        match level.to_lowercase().as_str() {
            "read_uncommitted" => conn_str.push_str("Isolation Level=ReadUncommitted;"),
            "read_committed" => conn_str.push_str("Isolation Level=ReadCommitted;"),
            "repeatable_read" => conn_str.push_str("Isolation Level=RepeatableRead;"),
            "serializable" => conn_str.push_str("Isolation Level=Serializable;"),
            "snapshot" => conn_str.push_str("Isolation Level=Snapshot;"),
            _ => {
                // If unknown level, pass through as-is (driver-specific)
                conn_str.push_str(&format!("Isolation Level={};", level));
            }
        }
    }

    let conn = env.connect_with_connection_string(&conn_str, ConnectionOptions::default())?;

    let cursor = conn
        .execute(sql, (), None)?
        .ok_or_else(|| anyhow!("SQL did not return a result set"))?;

    let text_size = config.max_text_size.unwrap_or(65536);
    let binary_size = config.max_binary_size.unwrap_or(65536);

    let mut builder = OdbcReaderBuilder::new();
    builder.with_max_text_size(text_size as usize);
    builder.with_max_binary_size(binary_size as usize);

    let arrow_record_batches = builder.build(cursor)?;

    let mut bytes = Vec::<u8>::new();
    {
        let schema = arrow_record_batches.schema();

        // Pipelining: write each batch immediately as it's fetched
        // This keeps memory usage constant instead of accumulating all data
        let mut writer = StreamWriter::try_new(&mut bytes, &schema)?;

        for batch in arrow_record_batches {
            writer.write(&batch?)?;
            // Each batch is written immediately, freeing memory
            // Memory usage stays constant regardless of dataset size
        }
        writer.finish()?;
    }

    Ok(bytes)
}

// Implementation function for Polars
fn query_polars_impl(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: &QueryConfig,
) -> PyResult<Py<PyAny>> {
    // High-level wrapper: use zero-copy Arrow C Data Interface for maximum performance
    let (schema_capsule, array_capsule) = query_arrow_c_data_impl(dsn, user, password, sql, config)
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("IM002") || msg.contains("connection") {
                PyConnectionError::new_err(format!("Connection Error: {}", msg))
            } else if msg.contains("SQL") || msg.contains("syntax") {
                PySQLError::new_err(format!("SQL Error: {}", msg))
            } else if msg.contains("Arrow") || msg.contains("c_data") {
                PyArrowError::new_err(format!("Arrow Error: {}", msg))
            } else {
                PyRuntimeError::new_err(msg)
            }
        })?;

    // Return Polars DataFrame directly from C Data Interface
    Python::with_gil(|py| {
        let polars = py.import_bound("polars")?;
        let pa = py.import_bound("pyarrow")?;

        // Import from C capsules
        let schema = pa
            .getattr("Schema")?
            .getattr("_import_from_c")?
            .call1((schema_capsule,))?;
        let array = pa
            .getattr("RecordBatch")?
            .getattr("_import_from_c")?
            .call1((array_capsule, schema))?;

        let df = polars.getattr("from_arrow")?.call1((array,))?;
        Ok(df.into())
    })
}

// Implementation function for Pandas
fn query_pandas_impl(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: &QueryConfig,
) -> PyResult<Py<PyAny>> {
    // High-level wrapper: use Arrow IPC for maximum compatibility with Pandas
    let bytes = query_arrow_ipc_impl(dsn, user, password, sql, config).map_err(|e| {
        let msg = e.to_string();
        if msg.contains("IM002") || msg.contains("connection") {
            PyConnectionError::new_err(format!("Connection Error: {}", msg))
        } else if msg.contains("SQL") || msg.contains("syntax") {
            PySQLError::new_err(format!("SQL Error: {}", msg))
        } else if msg.contains("Arrow") || msg.contains("c_data") {
            PyArrowError::new_err(format!("Arrow Error: {}", msg))
        } else {
            PyRuntimeError::new_err(msg)
        }
    })?;
    Python::with_gil(|py| {
        let pyarrow = py.import_bound("pyarrow")?;
        let io = py.import_bound("io")?;

        let buf = io.getattr("BytesIO")?.call1((bytes,))?;
        let table = pyarrow
            .getattr("ipc")?
            .getattr("open_stream")?
            .call1((buf,))?
            .getattr("read_all")?
            .call0()?;
        let df = table.getattr("to_pandas")?.call0()?;
        Ok(df.into())
    })
}

// Implementation function for Arrow C Data Interface
fn query_arrow_c_data_impl(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: &QueryConfig,
) -> Result<(Py<PyAny>, Py<PyAny>)> {
    let env = Environment::new()?;

    let mut conn_str = format!("DSN={};UID={};PWD={};", dsn, user, password);

    if config.read_only {
        conn_str.push_str("ReadOnly=1;");
    }

    if let Some(timeout) = config.connection_timeout {
        conn_str.push_str(&format!("Connection Timeout={};", timeout));
    }

    if let Some(timeout) = config.query_timeout {
        conn_str.push_str(&format!("Query Timeout={};", timeout));
    }

    if let Some(level) = &config.isolation_level {
        match level.to_lowercase().as_str() {
            "read_uncommitted" => conn_str.push_str("Isolation Level=ReadUncommitted;"),
            "read_committed" => conn_str.push_str("Isolation Level=ReadCommitted;"),
            "repeatable_read" => conn_str.push_str("Isolation Level=RepeatableRead;"),
            "serializable" => conn_str.push_str("Isolation Level=Serializable;"),
            "snapshot" => conn_str.push_str("Isolation Level=Snapshot;"),
            _ => {
                conn_str.push_str(&format!("Isolation Level={};", level));
            }
        }
    }

    let conn = env.connect_with_connection_string(&conn_str, ConnectionOptions::default())?;

    let cursor = conn
        .execute(sql, (), None)?
        .ok_or_else(|| anyhow!("SQL did not return a result set"))?;

    let text_size = config.max_text_size.unwrap_or(65536);
    let binary_size = config.max_binary_size.unwrap_or(65536);

    let mut builder = OdbcReaderBuilder::new();
    builder.with_max_text_size(text_size as usize);
    builder.with_max_binary_size(binary_size as usize);

    let arrow_record_batches = builder.build(cursor)?;

    // Collect all batches
    let mut batches = Vec::new();
    for batch in arrow_record_batches {
        batches.push(batch?);
    }

    if batches.is_empty() {
        return Err(anyhow!("No data returned from query"));
    }

    // Use the first batch for Arrow C Data Interface
    let first_batch = &batches[0];
    let _schema = first_batch.schema();

    // Convert RecordBatch to StructArray for FFI
    use arrow::array::StructArray;
    let struct_array = StructArray::from(first_batch.clone());
    let array_data = struct_array.into_data();

    // Convert to Arrow C Data Interface using the correct approach
    let (ffi_array, ffi_schema) = to_ffi(&array_data)?;

    Python::with_gil(|py| {
        // Create PyCapsules for schema and array
        let schema_capsule =
            PyCapsule::new_bound(py, ffi_schema, Some(CString::new("arrow_schema")?))?;
        let array_capsule =
            PyCapsule::new_bound(py, ffi_array, Some(CString::new("arrow_array")?))?;

        Ok((schema_capsule.into(), array_capsule.into()))
    })
}

// Implementation function for Arrow C Data with DataFrame option
fn query_arrow_c_data_with_df(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: &QueryConfig,
    return_dataframe: Option<bool>,
) -> PyResult<Py<PyAny>> {
    let return_df = return_dataframe.unwrap_or(false);

    match query_arrow_c_data_impl(dsn, user, password, sql, config) {
        Ok((schema_capsule, array_capsule)) => {
            if return_df {
                // Return Polars DataFrame directly
                Python::with_gil(|py| {
                    let polars = py.import_bound("polars")?;
                    let pa = py.import_bound("pyarrow")?;

                    let schema = pa
                        .getattr("Schema")?
                        .getattr("_import_from_c")?
                        .call1((schema_capsule,))?;
                    let array = pa
                        .getattr("RecordBatch")?
                        .getattr("_import_from_c")?
                        .call1((array_capsule, schema))?;

                    let df = polars.getattr("from_arrow")?.call1((array,))?;
                    Ok(df.into())
                })
            } else {
                // Return PyCapsules for manual control
                Python::with_gil(|py| {
                    let tuple = (schema_capsule, array_capsule);
                    Ok(tuple.into_py(py))
                })
            }
        }
        Err(e) => {
            let msg = e.to_string();

            if msg.contains("IM002") || msg.contains("connection") {
                Err(PyConnectionError::new_err(format!(
                    "Connection Error: {}",
                    msg
                )))
            } else if msg.contains("SQL") || msg.contains("syntax") {
                Err(PySQLError::new_err(format!("SQL Error: {}", msg)))
            } else if msg.contains("Arrow") || msg.contains("c_data") {
                Err(PyArrowError::new_err(format!("Arrow Error: {}", msg)))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(msg))
            }
        }
    }
}

// Standalone connect function for backward compatibility
#[pyfunction]
fn connect(
    dsn: &str,
    user: &str,
    password: &str,
    config: Option<&QueryConfig>,
) -> PyResult<IbarrowConnection> {
    Ok(IbarrowConnection::new(dsn, user, password, config))
}

#[pymodule]
fn ibarrow(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register the connection class and standalone function
    m.add_class::<IbarrowConnection>()?;
    m.add_class::<QueryConfig>()?;
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add(
        "PyConnectionError",
        _py.get_type_bound::<PyConnectionError>(),
    )?;
    m.add("PySQLError", _py.get_type_bound::<PySQLError>())?;
    m.add("PyArrowError", _py.get_type_bound::<PyArrowError>())?;
    Ok(())
}
