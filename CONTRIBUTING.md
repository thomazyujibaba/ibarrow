# Contributing to ibarrow

Thank you for your interest in contributing to ibarrow! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust toolchain (latest stable)
- Python 3.8+
- ODBC driver for your database
- Git

### Local Development

1. **Clone the repository:**
   ```bash
   git clone https://github.com/thomazyujibaba/ibarrow.git
   cd ibarrow
   ```

2. **Install development dependencies:**
   ```bash
   pip install maturin
   pip install -r requirements-dev.txt
   ```

3. **Build in development mode:**
   ```bash
   maturin develop
   ```

4. **Run tests:**
   ```bash
   pytest tests/
   ```

## Contributing Guidelines

### Code Style

- Follow Rust formatting with `cargo fmt`
- Follow Python PEP 8 for Python code
- Use meaningful commit messages
- Add tests for new features

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch:** `git checkout -b feature/your-feature-name`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run tests:** `pytest tests/`
6. **Commit changes:** `git commit -m "Add your feature"`
7. **Push to branch:** `git push origin feature/your-feature-name`
8. **Create a Pull Request**

### Testing

- All new features must include tests
- Tests should cover both success and error cases
- Use descriptive test names
- Mock external dependencies when possible

### Documentation

- Update README.md for user-facing changes
- Add docstrings for new Python functions
- Update type hints when adding parameters

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- Python version
- Rust version
- ODBC driver and version
- Database type and version
- Steps to reproduce
- Expected vs actual behavior
- Error messages (if any)

### Feature Requests

For feature requests, please include:

- Use case description
- Proposed solution
- Alternative solutions considered
- Additional context

## Release Process

Releases are managed through GitHub releases:

1. Update version in `Cargo.toml` and `pyproject.toml`
2. Update `CHANGELOG.md`
3. Create a GitHub release
4. CI/CD will automatically publish to PyPI

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Follow the golden rule

## Questions?

Feel free to open an issue for questions about contributing!
