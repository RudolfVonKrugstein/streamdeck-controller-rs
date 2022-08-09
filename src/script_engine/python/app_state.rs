use std::collections::HashMap;
use pyo3::prelude::*;
use std::sync::{Arc, RwLock};
use crate::config::hex_string_to_rgba_color;

/// Class for wrapping the app state to be used from python
#[pyclass]
pub struct AppState {
    state: Arc<RwLock<crate::state::AppState>>,
}

impl AppState {
    pub fn new(state: &Arc<RwLock<crate::state::AppState>>) -> AppState {
        AppState {
            state: state.clone(),
        }
    }
}

#[pymethods]
impl AppState {
    pub fn load_page(&self, page_name: String) {
        self.state.write().unwrap().load_page(&page_name).unwrap();
    }

    pub fn set_named_button_up_face(&self, button_name: String, properties: HashMap<String, String>) {
        self.state.write().unwrap().set_named_button_up_face(
            &button_name,
            match properties.get("color") {
                None => None,
                Some(c) => Some(hex_string_to_rgba_color(c).unwrap()),
            },
            properties.get("file").cloned(),
            properties.get("label").cloned(),
            match properties.get("labelcolor") {
                None => None,
                Some(c) => Some(hex_string_to_rgba_color(c).unwrap()),
            },
            properties.get("sublabel").cloned(),
            match properties.get("sublabelcolor") {
                None => None,
                Some(c) => Some(hex_string_to_rgba_color(c).unwrap()),
            },
            properties.get("superlabel").cloned(),
            match properties.get("superlabelcolor") {
                None => None,
                Some(c) => Some(hex_string_to_rgba_color(c).unwrap()),
            });
    }
}
