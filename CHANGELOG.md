# Changelog

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
