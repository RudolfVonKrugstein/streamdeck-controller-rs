use super::stdout::LoggingStdout;
use log::{error, info};
use pyo3::prelude::*;
use pyo3::Python;
use std::sync::Arc;

pub struct PythonEngine {}

impl PythonEngine {
    pub fn new() -> PythonEngine {
        PythonEngine {}
    }

    pub fn run_event_handler(
        &self,
        event_handler: &Arc<crate::state::EventHandler>,
    ) -> Result<(), PyErr> {
        match Python::with_gil(|py| -> Result<(), PyErr> {
            let sys = py.import("sys")?;
            sys.setattr("stdout", LoggingStdout.into_py(py))?;
            py.run(event_handler.script.as_str(), None, None)?;
            Ok(())
        }) {
            Ok(_) => {
                info!("python script finished successfully")
            }
            Err(e) => {
                Python::with_gil(|py| {
                    error!("python script failed: {}", e.value(py));
                });
            }
        };
        Ok(())
    }
}
