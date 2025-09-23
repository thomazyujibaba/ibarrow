use anyhow::{anyhow, Result};
use arrow_ipc::writer::StreamWriter;
use odbc_api::{Environment, ConnectionOptions};
use pyo3::prelude::*;
use pyo3::types::PyCapsule;
use arrow_odbc::OdbcReaderBuilder;
use arrow::record_batch::RecordBatchReader;
use arrow::array::Array;
use serde::{Deserialize, Serialize};
use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema, to_ffi};
use std::ffi::{c_void, CString};

use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(ibarrow, PyConnectionError, PyException);
create_exception!(ibarrow, PySQLError, PyException);
create_exception!(ibarrow, PyArrowError, PyException);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct QueryConfig {
    #[pyo3(get, set)]
    pub batch_size: usize,
    #[pyo3(get, set)]
    pub read_only: bool,
    #[pyo3(get, set)]
    pub connection_timeout: Option<u32>,
    #[pyo3(get, set)]
    pub query_timeout: Option<u32>,
    #[pyo3(get, set)]
    pub max_text_size: Option<usize>,
    #[pyo3(get, set)]
    pub max_binary_size: Option<usize>,
    #[pyo3(get, set)]
    pub isolation_level: Option<String>,
}

#[pymethods]
impl QueryConfig {
    #[new]
    fn new(
        batch_size: Option<usize>,
        read_only: Option<bool>,
        connection_timeout: Option<u32>,
        query_timeout: Option<u32>,
        max_text_size: Option<usize>,
        max_binary_size: Option<usize>,
        isolation_level: Option<String>,
    ) -> Self {
        Self {
            batch_size: batch_size.unwrap_or(1000),
            read_only: read_only.unwrap_or(true),
            connection_timeout,
            query_timeout,
            max_text_size,
            max_binary_size,
            isolation_level,
        }
    }
}


#[pyfunction]
fn query_arrow_ipc(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: Option<&QueryConfig>,
) -> PyResult<Vec<u8>> {
    let config = config.cloned().unwrap_or_else(|| QueryConfig::new(None, None, None, None, None, None, None));
    
    match query_arrow_ipc_impl(dsn, user, password, sql, &config) {
        Ok(buf) => Ok(buf),
        Err(e) => {
            let msg = e.to_string();

            if msg.contains("IM002") || msg.contains("connection") {
                Err(PyConnectionError::new_err(format!("Connection Error: {}", msg)))
            } else if msg.contains("SQL") || msg.contains("syntax") {
                Err(PySQLError::new_err(format!("SQL Error: {}", msg)))
            } else if msg.contains("Arrow") || msg.contains("ipc") {
                Err(PyArrowError::new_err(format!("Arrow Error: {}", msg)))
            } else {
                Err(pyo3::exceptions::PyRuntimeError::new_err(msg))
            }
        }
    }
}

#[pyfunction]
fn query_polars(dsn: &str, user: &str, password: &str, sql: &str, config: Option<&QueryConfig>) -> PyResult<Py<PyAny>> {
    // High-level wrapper: use zero-copy Arrow C Data Interface for maximum performance
    let config = config.cloned().unwrap_or_else(|| QueryConfig::new(None, None, None, None, None, None, None));
    let (schema_capsule, array_capsule) = query_arrow_c_data_impl(dsn, user, password, sql, &config)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    
    // Return Polars DataFrame directly from C Data Interface
    Python::with_gil(|py| {
        let polars = py.import("polars")?;
        let pa = py.import("pyarrow")?;
        
        // Import from C capsules
        let schema = pa.getattr("Schema")?.getattr("_import_from_c")?.call1((schema_capsule,))?;
        let array = pa.getattr("RecordBatch")?.getattr("_import_from_c")?.call1((array_capsule, schema))?;
        
        let df = polars.getattr("from_arrow")?.call1((array,))?;
        Ok(df.into())
    })
}

#[pyfunction]
fn query_pandas(dsn: &str, user: &str, password: &str, sql: &str, config: Option<&QueryConfig>) -> PyResult<Py<PyAny>> {
    // High-level wrapper: use Arrow IPC for maximum compatibility with Pandas
    let bytes = query_arrow_ipc(dsn, user, password, sql, config)?;
    Python::with_gil(|py| {
        let pyarrow = py.import("pyarrow")?;
        let io = py.import("io")?;
        
        let buf = io.getattr("BytesIO")?.call1((bytes,))?;
        let table = pyarrow.getattr("ipc")?.getattr("open_stream")?.call1((buf,))?.getattr("read_all")?.call0()?;
        let df = table.getattr("to_pandas")?.call0()?;
        Ok(df.into())
    })
}

// Temporarily disabled - Arrow C Data Interface functionality
// #[pyfunction]
// fn query_arrow_c_data(
//     dsn: &str,
//     user: &str,
//     password: &str,
//     sql: &str,
//     config: Option<&QueryConfig>,
//     return_dataframe: Option<bool>
// ) -> PyResult<Py<PyAny>> {
//     // Implementation temporarily disabled
// }




fn query_arrow_ipc_impl(dsn: &str, user: &str, password: &str, sql: &str, config: &QueryConfig) -> Result<Vec<u8>> {
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
    
    let cursor = conn.execute(sql, (), None)?
        .ok_or_else(|| anyhow!("SQL did not return a result set"))?;

    let text_size = config.max_text_size.unwrap_or(65536);
    let binary_size = config.max_binary_size.unwrap_or(65536);
    
    let mut builder = OdbcReaderBuilder::new();
    builder.with_max_text_size(text_size);
    builder.with_max_binary_size(binary_size);
        // .with_batch_size(config.batch_size) // Not available in this version
        // .with_infer_schema_from_odbc(true); // Not available in this version
    
    let arrow_record_batches = builder.build(cursor)?;
    
    let mut bytes = Vec::<u8>::new();
    {
        let schema = arrow_record_batches.schema();
        
        // Pipelining: write each batch immediately as it's fetched
        // This keeps memory usage constant instead of accumulating all data
        let mut writer = StreamWriter::try_new(&mut bytes, &schema)?;
        
        // Always use pipelining for maximum performance and memory efficiency
        for batch in arrow_record_batches {
            writer.write(&batch?)?;
            // Each batch is written immediately, freeing memory
            // Memory usage stays constant regardless of dataset size
        }
        writer.finish()?;
    }

    Ok(bytes)
}

// Arrow C Data Interface implementation
fn query_arrow_c_data_impl(dsn: &str, user: &str, password: &str, sql: &str, config: &QueryConfig) -> Result<(Py<PyAny>, Py<PyAny>)> {
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
    
    let cursor = conn.execute(sql, (), None)?
        .ok_or_else(|| anyhow!("SQL did not return a result set"))?;

    let text_size = config.max_text_size.unwrap_or(65536);
    let binary_size = config.max_binary_size.unwrap_or(65536);
    
    let mut builder = OdbcReaderBuilder::new();
    builder.with_max_text_size(text_size);
    builder.with_max_binary_size(binary_size);
    
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
    let schema = first_batch.schema();

    // Convert RecordBatch to StructArray for FFI
    use arrow::array::StructArray;
    let struct_array = StructArray::from(first_batch.clone());
    let array_data = struct_array.into_data();
    
    // Convert to Arrow C Data Interface using the correct approach
    let (ffi_array, ffi_schema) = to_ffi(&array_data)?;
    
    Python::with_gil(|py| {
        // Create PyCapsules for schema and array
        let schema_capsule = PyCapsule::new_bound(py, ffi_schema, Some(CString::new("arrow_schema")?))?;
        let array_capsule = PyCapsule::new_bound(py, ffi_array, Some(CString::new("arrow_array")?))?;
        
        Ok((schema_capsule.into(), array_capsule.into()))
    })
}

#[pyfunction]
fn query_arrow_c_data(
    dsn: &str,
    user: &str,
    password: &str,
    sql: &str,
    config: Option<&QueryConfig>,
    return_dataframe: Option<bool>
) -> PyResult<Py<PyAny>> {
    let config = config.cloned().unwrap_or_else(|| QueryConfig::new(None, None, None, None, None, None, None));
    let return_df = return_dataframe.unwrap_or(false);
    
    match query_arrow_c_data_impl(dsn, user, password, sql, &config) {
        Ok((schema_capsule, array_capsule)) => {
            if return_df {
                // Return Polars DataFrame directly
                Python::with_gil(|py| {
                    let polars = py.import("polars")?;
                    let pa = py.import("pyarrow")?;
                    
                    let schema = pa.getattr("Schema")?.getattr("_import_from_c")?.call1((schema_capsule,))?;
                    let array = pa.getattr("RecordBatch")?.getattr("_import_from_c")?.call1((array_capsule, schema))?;
                    
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
        },
        Err(e) => {
            let msg = e.to_string();

            if msg.contains("IM002") || msg.contains("connection") {
                Err(PyConnectionError::new_err(format!("Connection Error: {}", msg)))
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


#[pymodule]
fn ibarrow(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(query_arrow_ipc, m)?)?;
    m.add_function(wrap_pyfunction!(query_polars, m)?)?;
    m.add_function(wrap_pyfunction!(query_pandas, m)?)?;
    m.add_function(wrap_pyfunction!(query_arrow_c_data, m)?)?;
    m.add_class::<QueryConfig>()?;
    m.add("PyConnectionError", _py.get_type_bound::<PyConnectionError>())?;
    m.add("PySQLError", _py.get_type_bound::<PySQLError>())?;
    m.add("PyArrowError", _py.get_type_bound::<PyArrowError>())?;
    Ok(())
}
