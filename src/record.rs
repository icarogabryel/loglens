use chrono::NaiveDateTime;
use pyo3::prelude::*;

#[derive(Clone, Copy)]
pub(crate) enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
        }
    }

    pub(crate) fn parse(value: &str) -> Self {
        match value {
            "DEBUG" => Self::Debug,
            "INFO" => Self::Info,
            "WARNING" => Self::Warning,
            "ERROR" => Self::Error,
            "CRITICAL" => Self::Critical,
            _ => unreachable!("Invalid log level"),
        }
    }
}

#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct LogRecord {
    pub(crate) timestamp: NaiveDateTime,
    pub(crate) level: LogLevel,
    pub(crate) logger: String,
    pub(crate) message: String,
}

#[pymethods]
impl LogRecord {
    #[getter]
    fn timestamp(&self) -> String {
        self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    #[getter]
    fn level(&self) -> &'static str {
        self.level.as_str()
    }

    #[getter]
    fn logger(&self) -> &str {
        &self.logger
    }

    #[getter]
    fn message(&self) -> &str {
        &self.message
    }
}
