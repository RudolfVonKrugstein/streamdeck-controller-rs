use super::error::Error;
use crate::config;
use crate::state::button_face::ButtonFace;
use crate::state::defaults::Defaults;
use crate::state::event_handler::EventHandler;
use std::collections::HashMap;
use std::sync::Arc;
use streamdeck_hid_rs::StreamDeckType;

/// Everything that belong to setup a button.
/// This is not the state of a button, but the setup.
/// This setup can be applied to any button. But it is not
/// the state of concrete button, it is part of the state (see [ButtonState]).
pub struct ButtonSetup {
    pub up_face: Option<Arc<ButtonFace>>,
    pub down_face: Option<Arc<ButtonFace>>,
    pub up_handler: Option<Arc<EventHandler>>,
    pub down_handler: Option<Arc<EventHandler>>,
}

impl ButtonSetup {
    /// Create Button Setup from configuration.
    ///
    /// # Arguments
    ///
    /// device_type - The type of Streamdeck for which this [ButtonSetup] is created.
    /// config - The config to create the [ButtonSetup] from.
    ///
    /// # Return
    ///
    /// The created config, or the error if config could not be created.
    pub fn from_optional_name_config(
        device_type: &streamdeck_hid_rs::StreamDeckType,
        config: &config::ButtonConfigOptionalName,
        defaults: &Defaults,
    ) -> Result<ButtonSetup, Error> {
        // Create the members
        let up_face = match &config.up_face {
            None => None,
            Some(f) => Some(Arc::new(ButtonFace::from_config(device_type, f, defaults)?)),
        };
        let down_face = match &config.down_face {
            None => None,
            Some(f) => Some(Arc::new(ButtonFace::from_config(device_type, f, defaults)?)),
        };
        let up_handler = match &config.up_handler {
            None => None,
            Some(e) => Some(Arc::new(EventHandler::from_config(e)?)),
        };
        let down_handler = match &config.down_handler {
            None => None,
            Some(e) => Some(Arc::new(EventHandler::from_config(e)?)),
        };
        Ok(ButtonSetup {
            up_face,
            down_face,
            up_handler,
            down_handler,
        })
    }

    /// Create Button Setup from configuration.
    ///
    /// # Arguments
    ///
    /// device_type - The type of Streamdeck for which this [ButtonSetup] is created.
    /// config - The config to create the [ButtonSetup] from.
    ///
    /// # Return
    ///
    /// The created config, or the error if config could not be created.
    pub fn from_config_with_name(
        device_type: &streamdeck_hid_rs::StreamDeckType,
        config: &config::ButtonConfigWithName,
        defaults: &Defaults,
    ) -> Result<ButtonSetup, Error> {
        // Create the members
        let up_face = match &config.up_face {
            None => None,
            Some(f) => Some(Arc::new(ButtonFace::from_config(device_type, f, defaults)?)),
        };
        let down_face = match &config.down_face {
            None => None,
            Some(f) => Some(Arc::new(ButtonFace::from_config(device_type, f, defaults)?)),
        };
        let up_handler = match &config.up_handler {
            None => None,
            Some(e) => Some(Arc::new(EventHandler::from_config(e)?)),
        };
        let down_handler = match &config.down_handler {
            None => None,
            Some(e) => Some(Arc::new(EventHandler::from_config(e)?)),
        };
        Ok(ButtonSetup {
            up_face,
            down_face,
            up_handler,
            down_handler,
        })
    }
}

/// The press state of a button.
#[derive(PartialEq, Clone)]
pub enum PressState {
    Down,
    Up,
}

/// The state of a button!
pub struct ButtonState {
    button_name: String,
    press_state: PressState,
    // And how it is rendered. Basically, if this is not the same
    // as the press_state the button is not correctly rendered
    render_state: Option<PressState>,
}

impl ButtonState {
    pub fn new(button_name: String) -> ButtonState {
        ButtonState {
            button_name,
            press_state: PressState::Up,
            render_state: None,
        }
    }

    pub fn empty() -> ButtonState {
        ButtonState {
            button_name: String::from("empty"),
            press_state: PressState::Up,
            render_state: None,
        }
    }

    /// Set, that it needs rendering
    pub fn set_needs_rendering(&mut self) {
        self.render_state = None;
    }

    /// Sets the press state of the button
    pub fn set_pressed(
        &mut self,
        named_buttons: &HashMap<String, Arc<ButtonSetup>>,
    ) -> Option<Arc<EventHandler>> {
        self.press_state = PressState::Down;
        self.get_setup(named_buttons)
            .and_then(|s| s.down_handler.clone())
    }

    /// Sets the press state of the button
    pub fn set_released(
        &mut self,
        named_buttons: &HashMap<String, Arc<ButtonSetup>>,
    ) -> Option<Arc<EventHandler>> {
        self.press_state = PressState::Up;
        self.get_setup(named_buttons)
            .and_then(|s| s.up_handler.clone())
    }

    /// Returns whether the button needs rendering
    pub fn needs_rendering(&self) -> bool {
        if let Some(rs) = &self.render_state {
            return *rs != self.press_state;
        }
        true
    }

    /// Get the ButtonSetup, either from the internal setup
    /// or from the list of global setups
    fn get_setup(
        &self,
        named_buttons: &HashMap<String, Arc<ButtonSetup>>,
    ) -> Option<Arc<ButtonSetup>> {
        named_buttons.get(&self.button_name).cloned()
    }

    /// Sets/changes the setup for this button!
    pub fn set_button(&mut self, name: String) {
        self.button_name = name;
        self.render_state = None;
    }

    /// Sets the button to rendered and gets the faced that has to be rendered
    /// # Return
    ///
    /// None - if no rendering is needed.
    /// Some(...) - The button face for rendering on this button.
    pub fn set_rendered_and_get_face_for_rendering(
        &mut self,
        named_buttons: &HashMap<String, Arc<ButtonSetup>>,
    ) -> Option<Arc<ButtonFace>> {
        if self.needs_rendering() {
            self.render_state = Some(self.press_state.clone());
            let setup = self.get_setup(named_buttons)?;
            match self.press_state {
                PressState::Up => match setup.up_face {
                    None => setup.down_face.clone(),
                    Some(_) => setup.up_face.clone(),
                },
                PressState::Down => match setup.down_face {
                    None => setup.up_face.clone(),
                    Some(_) => setup.down_face.clone(),
                },
            }
        } else {
            None
        }
    }

    /// Tests the button name
    pub fn uses_button(&self, name: &String) -> bool {
        self.button_name.eq(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ButtonConfigOptionalName;

    #[test]
    fn per_default_the_button_needs_rendering() {
        // Setup
        let state = ButtonState::new("button".to_string());

        // Act

        // Test
        assert!(state.needs_rendering());
    }

    #[test]
    fn get_correct_setup_on_named_button() {
        // Setup
        let state = ButtonState::new("button".to_string());
        let mut named_buttons = HashMap::new();
        let setup = Arc::new(ButtonSetup {
            up_face: None,
            down_face: None,
            up_handler: None,
            down_handler: None,
        });
        named_buttons.insert(String::from("button"), setup.clone());

        // Act
        let returned_face = state.get_setup(&named_buttons);

        // Test
        assert!(returned_face.is_some());
        let returned_face_unwrap = returned_face.unwrap();
        assert!(Arc::ptr_eq(&returned_face_unwrap, &setup));
    }

    #[test]
    fn after_getting_button_face_and_set_rendered_no_rendering_needed() {
        // Setup
        let mut state = ButtonState::new("button".to_string());
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Arc::new(ButtonSetup {
                up_face: None,
                down_face: None,
                up_handler: None,
                down_handler: None,
            }),
        );

        // Act
        state.set_rendered_and_get_face_for_rendering(&named_buttons);

        // Test
        assert!(!state.needs_rendering());
    }

    #[test]
    fn when_changing_button_is_pressed_rendering_is_needed_again() {
        // Setup
        let mut state = ButtonState::new("button".to_string());
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Arc::new(ButtonSetup {
                up_face: None,
                down_face: None,
                up_handler: None,
                down_handler: None,
            }),
        );

        // Act
        state.set_rendered_and_get_face_for_rendering(&named_buttons);
        state.set_pressed(&named_buttons);

        // Test
        assert!(state.needs_rendering());
    }

    #[test]
    fn when_changing_button_is_released_rendering_is_needed_again() {
        // Setup
        let mut state = ButtonState::new("button".to_string());
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Arc::new(ButtonSetup {
                up_face: None,
                down_face: None,
                up_handler: None,
                down_handler: None,
            }),
        );

        // Act
        state.set_pressed(&named_buttons);
        state.set_rendered_and_get_face_for_rendering(&named_buttons);
        state.set_released(&named_buttons);

        // Test
        assert!(state.needs_rendering());
    }

    #[test]
    fn when_changing_the_setup_rendering_is_needed_again() {
        // Setup
        let mut state = ButtonState::new("button".to_string());

        // Act
        state.set_button("button2".to_string());

        // Test
        assert!(state.needs_rendering());
    }
}
