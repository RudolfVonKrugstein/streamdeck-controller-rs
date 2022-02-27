use super::button::ButtonSetup;
use super::button::ButtonState;
use super::error::Error;
use super::page::Page;
use crate::config;
use crate::state::button_face::ButtonFace;
use crate::state::event_handler::EventHandler;
use std::collections::HashMap;
use std::rc::Rc;
use streamdeck_hid_rs::StreamDeckType;

/// The complete app state!
struct AppState {
    /// Named buttons, that can be used and modified
    named_buttons: HashMap<String, Rc<ButtonSetup>>,
    /// Pages, that can be loaded
    pages: HashMap<String, Rc<Page>>,
    /// The current loaded buttons
    buttons: Vec<ButtonState>,
    /// The device type this is for!
    device_type: StreamDeckType,
}

impl AppState {
    /// Create an app state from configuration
    ///
    /// # Arguments
    ///
    /// device_type - The type of Stremdeck device we create this for!
    /// config - Loaded configurations object
    ///
    /// # Result
    ///
    /// If the configuration is ok, the App state. Otherwise the error that occurred during
    /// creation of the state from the config.
    pub fn from_config(
        device_type: &StreamDeckType,
        config: &config::Config,
    ) -> Result<AppState, Error> {
        let mut named_buttons: HashMap<String, Rc<ButtonSetup>> = HashMap::new();

        if let Some(config_buttons) = &config.buttons {
            for button_config in config_buttons {
                named_buttons.insert(
                    button_config.name.clone(),
                    Rc::new(
                        ButtonSetup::from_config_with_name(&StreamDeckType::Orig, &button_config)
                            .unwrap(),
                    ),
                );
            }
        }

        let mut pages: HashMap<String, Rc<Page>> = HashMap::new();

        for page_config in &config.pages {
            let (page, more_named_buttons) =
                Page::from_config_with_named_buttons(device_type, &page_config)?;
            pages.insert(page_config.name.clone(), Rc::new(page));
            named_buttons.extend(more_named_buttons);
        }

        let mut buttons = Vec::new();
        for _ in 0..device_type.total_num_buttons() {
            buttons.push(ButtonState::empty());
        }

        Ok(AppState {
            named_buttons,
            pages,
            buttons,
            device_type: device_type.clone(),
        })
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
    fn on_button_pressed(&mut self, button_id: usize) -> Option<Rc<EventHandler>> {
        let mut button = self.buttons.get_mut(button_id)?;
        button.set_pressed(&self.named_buttons)
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
    fn on_button_released(&mut self, button_id: usize) -> Option<Rc<EventHandler>> {
        let mut button = self.buttons.get_mut(button_id)?;
        button.set_released(&self.named_buttons)
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
        // Find the page
        let page = self
            .pages
            .get(&page_name)
            .ok_or(Error::PageNotFound(page_name))?;

        // Load all the buttons
        for button in &page.buttons {
            self.buttons[button.position.to_button_index(&self.device_type)]
                .set_setup(&button.setup);
        }

        // All went fine!
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns a full config to be used in tests
    ///
    /// The config contains 1 page with all buttons!
    fn get_full_config() -> config::Config {
        let mut named_buttons = Vec::new();
        for i in 0..5 {
            named_buttons.push(config::ButtonConfigWithName {
                name: format!("named_button{}", i),
                up_face: Some(config::ButtonFaceConfig {
                    color: Some(config::ColorConfig::HEXString("#FF0000".to_string())),
                    file: None,
                    label: None,
                    sublabel: None,
                    superlabel: None,
                }),
                down_face: None,
                up_handler: Some(config::EventHandlerConfig::AsCode {
                    code: format!("on_named_button_{}_up", i),
                }),
                down_handler: Some(config::EventHandlerConfig::AsCode {
                    code: format!("on_named_button_{}_down", i),
                }),
            });
        }

        let mut page_buttons = Vec::new();
        for i in 0..15 {
            page_buttons.push(config::PageButtonConfig {
                position: config::ButtonPositionConfig {
                    row: i / 5,
                    col: i % 5,
                },
                button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                    name: Some(format!("page_button{}", i)),
                    up_face: Some(config::ButtonFaceConfig {
                        color: None,
                        file: None,
                        label: None,
                        sublabel: None,
                        superlabel: None,
                    }),
                    down_face: None,
                    up_handler: Some(config::EventHandlerConfig::AsCode {
                        code: format!("on_page_button_{}_up", i),
                    }),
                    down_handler: Some(config::EventHandlerConfig::AsCode {
                        code: format!("on_page_button_{}_down", i),
                    }),
                }),
            });
        }
        config::Config {
            defaults: None,
            buttons: Some(named_buttons),
            pages: vec![config::PageConfig {
                name: "page1".to_string(),
                buttons: page_buttons,
            }],
        }
    }

    #[test]
    fn named_buttons_are_loaded_from_config() {
        // Setup
        let config = get_full_config();

        // Act
        let state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        //Test
        for i in 0..5 {
            assert!(state
                .named_buttons
                .get(&format!("named_button{}", i))
                .is_some());
        }
    }

    #[test]
    fn pages_are_loaded_from_config() {
        // Setup
        let config = get_full_config();

        // Act
        let state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        //Test
        assert!(state.pages.get(&String::from("page1")).is_some());
    }

    #[test]
    fn named_buttons_of_page_appear_in_named_buttons() {
        // Setup
        let config = get_full_config();

        // Act
        let state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        //Test
        for i in 0..15 {
            assert!(state
                .named_buttons
                .get(&format!("page_button{}", i))
                .is_some());
        }
    }

    #[test]
    fn correct_button_press_and_release_events_are_returned() {
        // Setup
        let config = get_full_config();

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.load_page(String::from("page1")).unwrap();
        let press_event = state.on_button_pressed(0).unwrap();
        let release_event = state.on_button_released(0).unwrap();

        //Test
        assert_eq!(press_event.script, String::from("on_page_button_4_down"));
        assert_eq!(release_event.script, String::from("on_page_button_4_up"));
    }

    #[test]
    #[ignore]
    fn after_loading_page_all_buttons_need_rendering() {
        todo!()
    }

    #[test]
    #[ignore]
    fn after_button_press_face_is_returned_for_rendering() {
        todo!()
    }

    #[test]
    #[ignore]
    fn after_button_release_face_is_returned_for_rendering() {
        todo!()
    }

    #[test]
    #[ignore]
    fn button_press_and_release_results_in_no_need_for_rendering() {
        todo!()
    }

    #[test]
    #[ignore]
    fn page_loading_results_in_face_for_new_button_returned_on_button_press() {
        todo!()
    }

    #[test]
    #[ignore]
    fn not_existing_page_loading_results_in_error() {
        todo!()
    }
}
