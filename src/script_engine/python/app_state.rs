use pyo3::prelude::*;

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
