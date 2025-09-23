"""
Basic tests for the ibarrow module.
"""

import pytest
import ibarrow


def test_module_import():
    """Test if the module can be imported correctly."""
    assert hasattr(ibarrow, "connect")
    assert hasattr(ibarrow, "IbarrowConnection")
    assert hasattr(ibarrow, "QueryConfig")


def test_exceptions_available():
    """Test if custom exceptions are available."""
    assert hasattr(ibarrow, "PyConnectionError")
    assert hasattr(ibarrow, "PySQLError")
    assert hasattr(ibarrow, "PyArrowError")


def test_connect_invalid_connection():
    """Test behavior with invalid connection."""
    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn", user="invalid_user", password="invalid_password"
        )
        conn.query_arrow_ipc("SELECT 1")


def test_query_polars_invalid_connection():
    """Test behavior with invalid connection for Polars."""
    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn", user="invalid_user", password="invalid_password"
        )
        conn.query_polars("SELECT 1")


def test_query_config_creation():
    """Test QueryConfig creation and usage."""
    config = ibarrow.QueryConfig(
        batch_size=2000,
        read_only=True,
        connection_timeout=30,
        query_timeout=60,
        max_text_size=32768,
        max_binary_size=16384,
        isolation_level="READ_COMMITTED",
    )

    assert config.batch_size == 2000
    assert config.read_only is True
    assert config.connection_timeout == 30
    assert config.query_timeout == 60
    assert config.max_text_size == 32768
    assert config.max_binary_size == 16384
    assert config.isolation_level == "READ_COMMITTED"


def test_query_with_config():
    """Test query with custom configuration."""
    config = ibarrow.QueryConfig(
        batch_size=500, read_only=True, connection_timeout=15, max_text_size=16384
    )

    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn",
            user="invalid_user",
            password="invalid_password",
            config=config,
        )
        conn.query_arrow_ipc("SELECT 1")


def test_query_pandas_invalid_connection():
    """Test query_pandas with invalid connection parameters."""
    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn", user="invalid_user", password="invalid_password"
        )
        conn.query_pandas("SELECT 1")


def test_query_arrow_c_data_invalid_connection():
    """Test query_arrow_c_data with invalid connection parameters."""
    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn", user="invalid_user", password="invalid_password"
        )
        conn.query_arrow_c_data("SELECT 1")


def test_query_arrow_c_data_with_dataframe_invalid_connection():
    """Test query_arrow_c_data with return_dataframe=True and invalid connection parameters."""
    with pytest.raises(ibarrow.PyConnectionError):
        conn = ibarrow.connect(
            dsn="invalid_dsn", user="invalid_user", password="invalid_password"
        )
        conn.query_arrow_c_data("SELECT 1", return_dataframe=True)
