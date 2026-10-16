use pyo3::prelude::*;

mod parser;
mod record;
mod report;

/// Python module implemented in Rust.
#[pymodule]
mod loglens {
    #[pymodule_export]
    use crate::record::LogRecord;

    #[pymodule_export]
    use crate::report::analyze;

    #[pymodule_export]
    use crate::report::Report;
}
