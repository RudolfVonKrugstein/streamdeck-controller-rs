use super::button::ButtonSetup;
use super::button::ButtonState;
use super::error::Error;
use super::page::Page;
use crate::config;
use crate::state::button_face::ButtonFace;
use crate::state::event_handler::EventHandler;
use std::collections::HashMap;
use std::rc::Rc;

/// The complete app state!
struct AppState {
    /// Named buttons, that can be used and modified
    named_buttons: HashMap<String, Rc<ButtonSetup>>,
    /// Pages, that can be loaded
    pages: HashMap<String, Rc<Page>>,
    /// The current loaded buttons
    buttons: Vec<ButtonState>,
}

impl AppState {
    /// Create an app state from configuration
    ///
    /// # Arguments
    ///
    /// config - Loaded configations object
    ///
    /// # Result
    ///
    /// If the configuration is ok, the App state. Otherwise the error that occurred during
    /// creation of the state from the config.
    pub fn from_config(config: &config::Config) -> Result<AppState, Error> {
        todo!()
    }

    /// Button gets pressed
    ///
    /// # Arguments
    ///
    /// button_id - The id of the button beeing pressed
    ///
    /// # Return
    ///
    /// Event handler, that should be executed as a result of the button press.
    fn on_button_pressed(&mut self, button_id: u8) -> Option<EventHandler> {
        todo!()
    }

    /// Button gets released
    ///
    /// # Arguments
    ///
    /// button_id - The id of the button being released
    ///
    /// # Return
    ///
    /// Event handler, that should be executed as a result of the button release.
    fn on_button_released(&mut self, button_id: u8) -> Option<Rc<EventHandler>> {
        todo!()
    }

    /// Get all faces, that need rendering. Also sets all buttons do being rendered.
    ///
    /// # Arguments
    ///
    /// # Return
    ///
    /// List of tuples with the id of the button to be rendered and the ButtonFace that
    /// should be rendered on the button.
    fn set_rendered_and_get_rendering_faces(&mut self) -> Vec<Rc<ButtonFace>> {
        todo!()
    }

    /// Loads a page, setting all the buttons.
    ///
    /// # Arguments
    ///
    /// page_name - Name of the page to be loaded.
    ///
    /// # Return
    ///
    /// () if all went ok, Error if the page is not found.
    fn load_page(&mut self, page_name: String) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
