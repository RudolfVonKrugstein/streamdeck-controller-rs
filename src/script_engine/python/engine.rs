use super::stdout::LoggingStdout;
use pyo3::prelude::*;
use pyo3::{PyResult, Python};
use std::sync::Arc;

pub struct PythonEngine {}

impl PythonEngine {
    pub fn new() -> PythonEngine {
        PythonEngine {}
    }

    pub fn run_event_handler(
        &self,
        event_handler: &Arc<crate::state::EventHandler>,
    ) -> PyResult<()> {
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            sys.setattr("stdout", LoggingStdout.into_py(py))?;
            py.run(event_handler.script.as_str(), None, None)?;
            Ok(())
        })
    }
}
