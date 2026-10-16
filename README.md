# LogLens

LogLens is a Python log analysis library powered by Rust. It provides a Python
interface for analyzing log files, combining a Python-friendly API with the
performance of native Rust code.

## Features

- **Rust-powered:** Native log processing implemented in Rust.
- **Python API:** Access analysis results through Python objects.
- **Lazy evaluation:** Analyze log files only when necessary.
- **Caching:** Reuse previously computed analysis results.
- **Type hints:** Support for Python type checkers.
- **Error handling:** Expose file-related errors as Python exceptions.

## Usage

```python
from loglens import analyze

report = analyze("application.log")

print(report.content)
```

The `analyze()` function receives a log file path and returns a `Report` object
containing the log content.

## Development

LogLens uses the following tools:

- **Rust:** Core implementation.
- **PyO3:** Rust-Python interoperability.
- **Maturin:** Native extension building and packaging.
- **pytest:** Python testing.

### Build

Build and install the package in the active Python environment:

```bash
maturin develop
```

This command requires Maturin and a suitable Rust toolchain.
