# Changelog

## [0.1.8]

### Fixed
- **CRITICAL**: Fixed InvalidFooter error by using valid schema instead of Schema::empty()
- **Stream Logic**: Corrected empty data handling - write empty batch BEFORE finish(), not after
- **Schema Validation**: Use proper schema with Int32 field for empty streams instead of completely empty schema
- **Writer Management**: Eliminated double-writer creation that was corrupting streams

### Technical Details
- Changed `Schema::empty()` to `Schema::new(vec![Field::new("empty", DataType::Int32, true)])` for cursor None case
- Fixed stream writing order: write empty batch first, then call finish() - not the reverse
- Removed problematic `bytes.clear()` and writer recreation logic
- Ensures single writer per stream to prevent corruption

## [0.1.7]

### Fixed
- **Critical Fix**: Return valid empty Arrow streams instead of errors when queries return no result set
- **Improved Error Handling**: `query_arrow_ipc_impl` now returns empty but valid Arrow streams instead of throwing errors for queries with no data
- **Better Compatibility**: Ensures Python consumers always receive valid Arrow data structures, even for empty results
- **Robust Stream Handling**: Fixed InvalidFooter errors by ensuring proper stream finalization in all cases

### Technical Details
- Modified `query_arrow_ipc_impl` to return empty Arrow streams with proper schema when `cursor` is `None`
- Maintained error handling for `query_arrow_c_data_impl` as it requires different return type
- Improved stream writer finalization to prevent InvalidFooter errors
- Enhanced compatibility with Firebird/InterBase queries that may not return result sets

## [0.1.6]

### Fixed
- Fixed InvalidFooter error in Arrow IPC stream by ensuring proper footer is written even when no data is returned
- Added empty record batch creation for queries with no results to maintain valid Arrow stream format
- Fixed schema ownership issues in empty batch creation

## [0.1.5]

### Fixed
- Fixed critical "a bytes-like object is required, not 'list'" error in query_arrow_ipc method by properly converting Vec<u8> to Python bytes

## [0.1.4]

### Fixed
- Fixed code formatting issues in Rust source code
- Removed trailing whitespace in lib.rs
- Fixed "a bytes-like object is required, not 'list'" error in query_polars and query_pandas functions
- Properly convert Vec<u8> to PyBytes for BytesIO compatibility
- Added close() method to IbarrowConnection for compatibility with database connection patterns
- Fixed "Nome de fonte de dados muito longo" error by automatically detecting file paths and long DSN names, converting them to elegant direct connection strings using DATABASE parameter for file paths
- Improved error handling for queries that return no result set (like SELECT 1 FROM RDB$DATABASE) with more descriptive error messages
- Added test_connection() method to IbarrowConnection for easy connection testing without complex queries


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

## [0.1.1]

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
