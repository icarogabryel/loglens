use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::parser::parse_lines;
use crate::record::LogRecord;

#[pyclass]
pub struct Report {
    records: Vec<LogRecord>,
}

#[pymethods]
impl Report {
    #[getter]
    fn records(&self, py: Python<'_>) -> PyResult<Vec<Py<LogRecord>>> {
        self.records
            .iter()
            .cloned()
            .map(|record| Py::new(py, record))
            .collect()
    }

    fn print(&self) {
        for record in &self.records {
            println!(
                "{} {} [{}] {}",
                record.timestamp.format("%Y-%m-%d %H:%M:%S"),
                record.level.as_str(),
                record.logger,
                record.message,
            );
        }
    }
}

/// Reads and parses a log file, returning a report.
#[pyfunction]
pub fn analyze(path: PathBuf) -> PyResult<Report> {
    let content = fs::read_to_string(&path).map_err(|error| {
        PyIOError::new_err(format!(
            "Could not read log file '{}': {error}",
            path.display()
        ))
    })?;

    let records = parse_lines(&content);

    Ok(Report { records })
}
