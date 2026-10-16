use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Represents a report generated from a log file.
#[pyclass]
struct Report {
    #[pyo3(get)]
    content: String,
}

/// Reads a log file and returns a report.
#[pyfunction]
fn analyze(path: PathBuf) -> PyResult<Report> {
    let content = fs::read_to_string(&path).map_err(|error| {
        PyIOError::new_err(format!(
            "Could not read log file '{}': {error}",
            path.display()
        ))
    })?;

    Ok(Report { content })
}

/// Python module implemented in Rust.
#[pymodule]
mod loglens {
    #[pymodule_export]
    use super::analyze;

    #[pymodule_export]
    use super::Report;
}
