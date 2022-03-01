use pyo3::prelude::*;

#[pyclass]
pub struct LoggingStdout;

#[pymethods]
impl LoggingStdout {
    fn write(&self, data: &str) {
        println!("python: {:?}", data);
    }
}
