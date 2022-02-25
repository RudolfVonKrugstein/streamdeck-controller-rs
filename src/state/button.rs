use super::error::Error;
use crate::config;
use crate::state::button_face::ButtonFace;
use crate::state::event_handler::EventHandler;
use std::collections::HashMap;
use std::rc::Rc;

/// Everything that belong to setup a button.
/// This is not the state of a button, but the setup.
/// This setup can be applied to any button. But it is not
/// the state of concrete button, it is part of the state (see [ButtonState]).
pub struct ButtonSetup {
    up_face: Option<Rc<ButtonFace>>,
    down_face: Option<Rc<ButtonFace>>,
    up_handler: Option<Rc<EventHandler>>,
    down_handler: Option<Rc<EventHandler>>,
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
    pub fn from_config(
        device_type: &streamdeck_hid_rs::StreamDeckType,
        config: &config::ButtonConfigWithName,
    ) -> Result<ButtonSetup, Error> {
        // Create the members
        let up_face = match &config.up_face {
            None => None,
            Some(f) => Some(Rc::new(ButtonFace::from_config(device_type, f)?)),
        };
        let down_face = match &config.down_face {
            None => None,
            Some(f) => Some(Rc::new(ButtonFace::from_config(device_type, f)?)),
        };
        let up_handler = match &config.up_handler {
            None => None,
            Some(e) => Some(Rc::new(EventHandler::from_config(e)?)),
        };
        let down_handler = match &config.down_handler {
            None => None,
            Some(e) => Some(Rc::new(EventHandler::from_config(e)?)),
        };
        Ok(ButtonSetup {
            up_face,
            down_face,
            up_handler,
            down_handler,
        })
    }
}

/// A button setup can either be referenced directly or via its name!
pub enum ButtonSetupOrName {
    Name(String),
    Setup(Rc<ButtonSetup>),
}

/// The press state of a button.
#[derive(PartialEq, Clone)]
pub enum PressState {
    Down,
    Up,
}

/// The state of a button!
pub struct ButtonState {
    setup: ButtonSetupOrName,
    press_state: PressState,
    // And how it is rendered. Basically, if this is not the same
    // as the press_state the button is not correctly rendered
    render_state: Option<PressState>,
}

impl ButtonState {
    pub fn new(setup: ButtonSetupOrName) -> ButtonState {
        ButtonState {
            setup,
            press_state: PressState::Up,
            render_state: None,
        }
    }

    /// Sets the press state of the button
    pub fn set_pressed(
        &mut self,
        named_buttons: &HashMap<String, Rc<ButtonSetup>>,
    ) -> Option<Rc<EventHandler>> {
        self.press_state = PressState::Down;
        self.get_setup(named_buttons)
            .and_then(|s| s.down_handler.clone())
    }

    /// Sets the press state of the button
    pub fn set_released(
        &mut self,
        named_buttons: &HashMap<String, Rc<ButtonSetup>>,
    ) -> Option<Rc<EventHandler>> {
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
        named_buttons: &HashMap<String, Rc<ButtonSetup>>,
    ) -> Option<Rc<ButtonSetup>> {
        match &self.setup {
            ButtonSetupOrName::Name(name) => named_buttons.get(name).cloned(),
            ButtonSetupOrName::Setup(setup) => Some(setup.clone()),
        }
    }

    /// Sets the button to rendered and gets the faced that has to be rendered
    /// # Return
    ///
    /// None - if no rendering is needed.
    /// Some(...) - The button face for rendering on this button.
    pub fn set_rendered_and_get_face_for_rendering(
        &mut self,
        named_buttons: &HashMap<String, Rc<ButtonSetup>>,
    ) -> Option<Rc<ButtonFace>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_default_the_button_needs_rendering() {
        // Setup
        let state = ButtonState::new(ButtonSetupOrName::Name("button".to_string()));

        // Act

        // Test
        assert!(state.needs_rendering());
    }

    #[test]
    fn get_correct_setup_on_named_button() {
        // Setup
        let state = ButtonState::new(ButtonSetupOrName::Name("button".to_string()));
        let mut named_buttons = HashMap::new();
        let setup = Rc::new(ButtonSetup {
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
        assert!(Rc::ptr_eq(&returned_face_unwrap, &setup));
    }

    #[test]
    fn after_getting_button_face_and_set_rendered_no_rendering_needed() {
        // Setup
        let mut state = ButtonState::new(ButtonSetupOrName::Name("button".to_string()));
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Rc::new(ButtonSetup {
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
        let mut state = ButtonState::new(ButtonSetupOrName::Name("button".to_string()));
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Rc::new(ButtonSetup {
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
        let mut state = ButtonState::new(ButtonSetupOrName::Name("button".to_string()));
        let mut named_buttons = HashMap::new();
        named_buttons.insert(
            String::from("button"),
            Rc::new(ButtonSetup {
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
}
