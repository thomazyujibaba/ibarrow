# Changelog

## [0.1.4]

### Fixed
- Fixed code formatting issues in Rust source code
- Removed trailing whitespace in lib.rs

## [0.1.3.2]

### Fixed
- Simplified polars.read_ipc call for better compatibility

## [0.1.3.1]

### Fixed
- Fixed code formatting with cargo fmt

## [0.1.3]

### Fixed
- Fixed query_polars to use Arrow IPC and update documentation
- Fixed wheel installation in CI to match Python version

## [0.1.1] - 2024-12-19

### Added
- Support for direct connection strings in ibarrow
- Updated README with direct connection string support documentation

### Fixed
- Fixed PyPI credentials for maturin publish
- Fixed ODBC installation in publish job
- Fixed artifact name conflict in CI workflow
- Recreated ci.yml with clean cross-platform wheel installation
- Fixed Windows wheel installation with Python glob
- Fixed Windows wheel installation in CI
- Fixed error handling in query_polars and query_pandas
- Fixed Rust code formatting
- Fixed publish workflow ODBC linking issues

## [0.1.0]

### Added
- Initial release of ibarrow
- High-performance ODBC to Arrow data conversion
- Two-level API (high-level wrappers + low-level functions)
- Zero-copy Arrow C Data Interface support
- Native type preservation (INT/DECIMAL/FLOAT → Arrow natives)
- Streaming pipelining for constant memory usage
- Support for Polars and Pandas DataFrames
- Comprehensive error handling with specific exception types
- Batch size configuration for performance tuning
- Connection timeout and query timeout support
- Transaction isolation level configuration
- Text and binary field size limits
- Read-only connection support
- Initial release
- Core ODBC to Arrow conversion functionality
- Python bindings with PyO3
- Rust backend for maximum performance


### Performance
- Zero-copy data transfer via Arrow C Data Interface
- Streaming processing for large datasets (constant memory usage)
- Native type preservation avoiding string conversions
- Batch processing to avoid row-by-row fetching
- Optimized compilation with LTO and target-cpu=native

### Security
- Safe PyCapsule management with Arc shared ownership
- No dangling pointer risks
- Automatic memory cleanup
