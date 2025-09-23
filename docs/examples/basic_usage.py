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
        print(f"✅ Credenciais carregadas de {env_file}")
    else:
        print(
            "⚠️  Arquivo .env não encontrado. Use credenciais hardcoded ou configure o .env"
        )
except ImportError:
    print("⚠️  python-dotenv não instalado. Use: pip install python-dotenv")

# Credenciais - usa variáveis de ambiente se disponível, senão usa valores de exemplo
DSN = os.getenv("DSN", "your_dsn")
DB_USER = os.getenv("DB_USER", "username")
DB_PASSWORD = os.getenv("DB_PASSWORD", "password")
TEST_SQL = os.getenv("TEST_SQL", "SELECT * FROM your_table LIMIT 1000")


def basic_polars_example():
    """Basic example using Polars (recommended for most users)."""

    # Create connection
    conn = ibarrow.connect(dsn=DSN, user=DB_USER, password=DB_PASSWORD)

    # Simple query with Polars DataFrame
    df = conn.query_polars(TEST_SQL)

    print(f"Retrieved {len(df)} rows")
    print(df.head())

    return df


def basic_pandas_example():
    """Basic example using Pandas DataFrame."""

    # Simple query with Pandas DataFrame
    df = ibarrow.query_pandas(
        dsn="your_dsn",
        user="username",
        password="password",
        sql="SELECT * FROM your_table LIMIT 1000",
    )

    print(f"Retrieved {len(df)} rows")
    print(df.head())

    return df


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

    # Query with custom configuration
    df = ibarrow.query_polars(
        dsn="your_dsn",
        user="username",
        password="password",
        sql="SELECT * FROM large_table",
        config=config,
    )

    return df


def raw_arrow_data_example():
    """Example using raw Arrow data for maximum control."""

    # Get raw Arrow IPC bytes
    arrow_bytes = ibarrow.query_arrow_ipc(
        dsn="your_dsn",
        user="username",
        password="password",
        sql="SELECT * FROM your_table",
    )

    # Convert to Polars manually
    df = pl.read_ipc(arrow_bytes)

    return df


def arrow_c_data_example():
    """Example using Arrow C Data Interface for zero-copy."""

    # Get raw Arrow C Data Interface capsules
    schema_capsule, array_capsule = ibarrow.query_arrow_c_data(
        dsn="your_dsn",
        user="username",
        password="password",
        sql="SELECT * FROM your_table",
    )

    # Convert to PyArrow Table using zero-copy
    import pyarrow as pa

    schema = pa.Schema._import_from_c(schema_capsule)
    array = pa.Array._import_from_c(array_capsule)
    table = pa.Table.from_arrays([array], schema=schema)

    # Convert to Polars
    df = pl.from_arrow(table)

    return df


def error_handling_example():
    """Example of proper error handling."""

    try:
        df = ibarrow.query_polars(
            dsn="invalid_dsn",
            user="username",
            password="password",
            sql="SELECT * FROM your_table",
        )
    except ibarrow.PyConnectionError as e:
        print(f"Connection failed: {e}")
    except ibarrow.PySQLError as e:
        print(f"SQL error: {e}")
    except ibarrow.PyArrowError as e:
        print(f"Arrow processing error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")


if __name__ == "__main__":
    # Run examples (uncomment the ones you want to test)

    # basic_polars_example()
    # basic_pandas_example()
    # advanced_configuration_example()
    # raw_arrow_data_example()
    # arrow_c_data_example()
    # error_handling_example()

    print("Examples ready to run!")
