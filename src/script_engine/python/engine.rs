use super::stdout::LoggingStdout;
use crate::AppState;
use log::{error, info};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};
use pyo3::Python;
use std::sync::{Arc, RwLock};

pub struct PythonEngine {
    locals: Py<PyDict>,
}

impl PythonEngine {
    pub fn new(app_state: &Arc<RwLock<AppState>>) -> PyResult<PythonEngine> {
        let locals = Python::with_gil(|py| {
            let locals = PyDict::new(py);
            locals.set_item("state", Py::new(py, super::app_state::AppState::new(app_state)).unwrap());
            locals.into_py(py)
        });
        Ok(PythonEngine {
            locals
        })
    }

    pub fn run_event_handler(
        &self,
        event_handler: &Arc<crate::state::EventHandler>
    ) -> Result<(), PyErr> {
        match Python::with_gil(|py| -> Result<(), PyErr> {
            let sys = py.import("sys")?;
            sys.setattr("stdout", LoggingStdout.into_py(py))?;

            py.run(event_handler.script.as_str(), Some(self.locals.as_ref(py)), None)?;
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
