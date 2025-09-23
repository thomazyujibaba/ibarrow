# API Reference

This section contains detailed API documentation for ibarrow.

## Functions

### High-Level API (Recommended)

#### `query_polars(dsn, user, password, sql, config=None)`
Direct conversion to Polars DataFrame with zero-copy performance.

**Parameters:**
- `dsn` (str): ODBC data source name
- `user` (str): Database username
- `password` (str): Database password
- `sql` (str): SQL query to execute
- `config` (QueryConfig, optional): Configuration object

**Returns:**
- `polars.DataFrame`: Polars DataFrame with query results

**Example:**
```python
import ibarrow

df = ibarrow.query_polars(
    dsn="your_dsn",
    user="username",
    password="password",
    sql="SELECT * FROM your_table"
)
```

#### `query_pandas(dsn, user, password, sql, config=None)`
Direct conversion to Pandas DataFrame with maximum compatibility.

**Parameters:**
- `dsn` (str): ODBC data source name
- `user` (str): Database username
- `password` (str): Database password
- `sql` (str): SQL query to execute
- `config` (QueryConfig, optional): Configuration object

**Returns:**
- `pandas.DataFrame`: Pandas DataFrame with query results

**Example:**
```python
import ibarrow

df = ibarrow.query_pandas(
    dsn="your_dsn",
    user="username",
    password="password",
    sql="SELECT * FROM your_table"
)
```

### Low-Level API (Advanced)

#### `query_arrow_ipc(dsn, user, password, sql, config=None)`
Returns raw Arrow IPC bytes for maximum compatibility.

**Parameters:**
- `dsn` (str): ODBC data source name
- `user` (str): Database username
- `password` (str): Database password
- `sql` (str): SQL query to execute
- `config` (QueryConfig, optional): Configuration object

**Returns:**
- `bytes`: Arrow IPC stream bytes

**Example:**
```python
import ibarrow
import polars as pl

arrow_bytes = ibarrow.query_arrow_ipc(
    dsn="your_dsn",
    user="username",
    password="password",
    sql="SELECT * FROM your_table"
)

df = pl.read_ipc(arrow_bytes)
```

#### `query_arrow_c_data(dsn, user, password, sql, config=None, return_dataframe=False)`
Returns Arrow C Data Interface capsules for zero-copy performance.

**Parameters:**
- `dsn` (str): ODBC data source name
- `user` (str): Database username
- `password` (str): Database password
- `sql` (str): SQL query to execute
- `config` (QueryConfig, optional): Configuration object
- `return_dataframe` (bool, optional): Return Polars DataFrame directly (default: False)

**Returns:**
- If `return_dataframe=False`: `(PyCapsule, PyCapsule)` - Schema and array capsules
- If `return_dataframe=True`: `polars.DataFrame` - Polars DataFrame

**Example:**
```python
import ibarrow
import pyarrow as pa

# Get capsules for manual control
schema_capsule, array_capsule = ibarrow.query_arrow_c_data(
    dsn="your_dsn",
    user="username",
    password="password",
    sql="SELECT * FROM your_table"
)

# Convert to PyArrow Table
schema = pa.Schema._import_from_c(schema_capsule)
array = pa.Array._import_from_c(array_capsule)
table = pa.Table.from_arrays([array], schema=schema)
```

## Classes

### `QueryConfig`
Configuration class for customizing query behavior.

**Parameters:**
- `batch_size` (int, optional): Number of rows per batch (default: 1000)
- `read_only` (bool, optional): Read-only connection (default: True)
- `connection_timeout` (int, optional): Connection timeout in seconds
- `query_timeout` (int, optional): Query timeout in seconds
- `max_text_size` (int, optional): Maximum text field size in bytes (default: 65536)
- `max_binary_size` (int, optional): Maximum binary field size in bytes (default: 65536)
- `isolation_level` (str, optional): Transaction isolation level

**Example:**
```python
config = ibarrow.QueryConfig(
    batch_size=2000,
    read_only=True,
    connection_timeout=30,
    query_timeout=60,
    isolation_level="READ_COMMITTED"
)
```

## Exceptions

### `PyConnectionError`
Raised when there are connection-related errors.

### `PySQLError`
Raised when there are SQL syntax or execution errors.

### `PyArrowError`
Raised when there are Arrow processing errors.

**Example:**
```python
try:
    df = ibarrow.query_polars(dsn, user, password, sql)
except ibarrow.PyConnectionError as e:
    print(f"Connection failed: {e}")
except ibarrow.PySQLError as e:
    print(f"SQL error: {e}")
except ibarrow.PyArrowError as e:
    print(f"Arrow error: {e}")
```
