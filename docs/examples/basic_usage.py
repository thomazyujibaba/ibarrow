"""
Basic usage examples for ibarrow.

This file demonstrates common usage patterns and best practices.
"""

import os
from pathlib import Path
import ibarrow
import polars as pl

# Carrega credenciais do .env se disponível
try:
    from dotenv import load_dotenv

    env_file = Path(__file__).parent.parent.parent / ".env"
    if env_file.exists():
        load_dotenv(env_file)
        print(f"✅ Credentials loaded from {env_file}")
    else:
        print("⚠️  .env file not found. Use hardcoded credentials or configure .env")
except ImportError:
    print("⚠️  python-dotenv not installed. Use: pip install python-dotenv")

# Credentials - use environment variables if available, otherwise use example values
DSN = os.getenv("DSN", "your_dsn")
DB_USER = os.getenv("DB_USER", "username")
DB_PASSWORD = os.getenv("DB_PASSWORD", "password")
TEST_SQL = os.getenv("TEST_SQL", "SELECT * FROM your_table LIMIT 1000")


def basic_polars_example():
    """Basic example using Polars (recommended for most users)."""

    # Create connection
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # Simple query with Polars DataFrame
        df = conn.query_polars(TEST_SQL)

        print(f"Retrieved {len(df)} rows")
        print(df.head())

        return df
    finally:
        # Always close the connection when done
        conn.close()


def basic_pandas_example():
    """Basic example using Pandas DataFrame."""

    # Create connection
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # Simple query with Pandas DataFrame
        df = conn.query_pandas(TEST_SQL)

        print(f"Retrieved {len(df)} rows")
        print(df.head())

        return df
    finally:
        # Always close the connection when done
        conn.close()


def advanced_configuration_example():
    """Example with advanced configuration."""

    # Create custom configuration
    config = ibarrow.QueryConfig(
        batch_size=5000,  # Larger batches for better performance
        read_only=True,  # Read-only connection
        connection_timeout=30,  # 30 second connection timeout
        query_timeout=120,  # 2 minute query timeout
        max_text_size=65536,  # 64KB text fields
        max_binary_size=32768,  # 32KB binary fields
        isolation_level="READ_COMMITTED",  # Transaction isolation
    )

    # Create connection with custom configuration
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD, config=config)

    try:
        # Query with custom configuration
        df = conn.query_polars("SELECT * FROM large_table")

        return df
    finally:
        # Always close the connection when done
        conn.close()


def raw_arrow_data_example():
    """Example using raw Arrow data for maximum control."""

    # Create connection
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # Get raw Arrow IPC bytes (now returns proper Python bytes)
        arrow_bytes = conn.query_arrow_ipc("SELECT * FROM your_table")

        # Convert to Polars manually (now works directly)
        df = pl.read_ipc(arrow_bytes)

        return df
    finally:
        # Always close the connection when done
        conn.close()


def arrow_c_data_example():
    """Example using Arrow C Data Interface for zero-copy."""

    # Create connection
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # Get raw Arrow C Data Interface capsules
        schema_capsule, array_capsule = conn.query_arrow_c_data(
            "SELECT * FROM your_table"
        )

        # Convert to PyArrow Table using zero-copy
        import pyarrow as pa

        schema = pa.Schema._import_from_c(schema_capsule)
        array = pa.Array._import_from_c(array_capsule)
        table = pa.Table.from_arrays([array], schema=schema)

        # Convert to Polars
        df = pl.from_arrow(table)

        return df
    finally:
        # Always close the connection when done
        conn.close()


def error_handling_example():
    """Example of proper error handling."""

    conn = None
    try:
        # Create connection
        conn = ibarrow.connect(dsn="invalid_dsn", user="username", password="password")

        # Try to query
        df = conn.query_polars("SELECT * FROM your_table")

    except ibarrow.PyConnectionError as e:
        print(f"Connection failed: {e}")
    except ibarrow.PySQLError as e:
        print(f"SQL error: {e}")
    except ibarrow.PyArrowError as e:
        print(f"Arrow processing error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")
    finally:
        # Always close the connection if it was created
        if conn is not None:
            conn.close()


def long_dsn_name_example():
    """Example showing how ibarrow handles long DSN names and file paths automatically."""

    # Common scenarios that cause "Nome de fonte de dados muito longo" error:

    # 1. File paths (most common case)
    file_path_dsn = (
        r"C:\Program Files\Firebird\databases\my_very_long_database_name.fdb"
    )

    # 2. Long DSN names
    long_dsn = "very_long_database_source_name_that_exceeds_odbc_limit"

    # Both will be automatically converted to direct connection strings
    # File paths use DATABASE parameter, long DSNs use DSN parameter

    for dsn_name, description in [
        (file_path_dsn, "file path"),
        (long_dsn, "long DSN name"),
    ]:
        print(f"\n🔗 Testing {description}: {dsn_name}")

        conn = ibarrow.connect(dsn=dsn_name, user=DB_USER, password=DB_PASSWORD)

        try:
            # This will work automatically - no manual conversion needed
            df = conn.query_polars("SELECT * FROM your_table LIMIT 10")
            print(f"✅ Successfully connected with {description}")
            print(f"Retrieved {len(df)} rows")
        except Exception as e:
            print(f"❌ Connection failed: {e}")
        finally:
            conn.close()


def test_connection_example():
    """Test connection using the built-in test_connection() method."""

    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # Test connection with built-in method
        is_connected = conn.test_connection()

        if is_connected:
            print("✅ Connection test successful!")
            print("Database is accessible and credentials are valid.")

            # Now you can safely run queries
            df = conn.query_polars("SELECT 1 as test_value")
            print(f"Query result: {df}")
            return df
        else:
            print("❌ Connection test failed!")
            print("This may indicate:")
            print("- Database connection issue")
            print("- Database file not found")
            print("- Invalid credentials")
            print("- Database server not running")
            return None

    except Exception as e:
        print(f"❌ Connection test error: {e}")
        return None
    finally:
        conn.close()


def test_rdb_database_query():
    """Test querying RDB$DATABASE to verify connection."""

    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    try:
        # This query should work if connection is successful
        df = conn.query_polars("SELECT 1 FROM RDB$DATABASE")
        print(f"✅ RDB$DATABASE query successful: {len(df)} rows")
        print(df.head())
        return df
    except Exception as e:
        print(f"❌ RDB$DATABASE query failed: {e}")
        print("This may indicate:")
        print("- Database connection issue")
        print("- Database file not found")
        print("- Invalid credentials")
        print("- Database server not running")
        return None
    finally:
        conn.close()


if __name__ == "__main__":
    # Run examples (uncomment the ones you want to test)

    # basic_polars_example()
    # basic_pandas_example()
    # advanced_configuration_example()
    # raw_arrow_data_example()
    # arrow_c_data_example()
    # error_handling_example()
    # long_dsn_name_example()
    # test_connection_example()
    # test_rdb_database_query()

    print("Examples ready to run!")
