use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::parser::parse_line;
use crate::record::LogRecord;

type RecordCache = Arc<Mutex<Vec<Option<LogRecord>>>>;

#[pyclass]
pub struct Report {
    lines: Arc<Vec<String>>,
    indices: Vec<usize>,
    cache: RecordCache,
}

impl Report {
    fn get_record(&self, index: usize) -> LogRecord {
        let mut cache = self.cache.lock().unwrap();

        cache[index]
            .get_or_insert_with(|| parse_line(&self.lines[index]))
            .clone()
    }
}

#[pymethods]
impl Report {
    #[getter]
    fn records(&self, py: Python<'_>) -> PyResult<Vec<Py<LogRecord>>> {
        self.indices
            .iter()
            .map(|&index| Py::new(py, self.get_record(index)))
            .collect()
    }

    fn print(&self) {
        for &index in &self.indices {
            let record = self.get_record(index);

            println!(
                "{} {} [{}] {}",
                record.timestamp.format("%Y-%m-%d %H:%M:%S"),
                record.level.as_str(),
                record.logger,
                record.message,
            );
        }
    }

    #[pyo3(signature = (level=None, logger=None, text=None))]
    fn filter(
        &self,
        level: Option<&str>,
        logger: Option<&str>,
        text: Option<&str>,
    ) -> Self {
        let indices = self
            .indices
            .iter()
            .copied()
            .filter(|&index| {
                let record = self.get_record(index);

                level.is_none_or(|value| record.level.as_str() == value)
                    && logger.is_none_or(|value| record.logger == value)
                    && text.is_none_or(|value| record.message.contains(value))
            })
            .collect();

        Self {
            lines: Arc::clone(&self.lines),
            indices,
            cache: Arc::clone(&self.cache),
        }
    }
}

#[pyfunction]
pub fn analyze(path: PathBuf) -> PyResult<Report> {
    let content = fs::read_to_string(&path).map_err(|error| {
        PyIOError::new_err(format!(
            "Could not read log file '{}': {error}",
            path.display()
        ))
    })?;

    let lines: Vec<String> = content.lines().map(str::to_owned).collect();
    let cache = (0..lines.len()).map(|_| None).collect();

    Ok(Report {
        indices: (0..lines.len()).collect(),
        lines: Arc::new(lines),
        cache: Arc::new(Mutex::new(cache)),
    })
}
