use pyo3::prelude::*;
use std::sync::RwLock;

/// Class for wrapping the app state to be used from python
#[pyclass]
struct AppState {
    // state: RwLock<crate::state::AppState>,
}

impl AppState {
    // fn new(state: RwLock<crate::state::AppState>) -> AppState {
    //     AppState { state }
    // }
}
