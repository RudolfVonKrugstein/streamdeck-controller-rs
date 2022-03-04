use super::button::ButtonSetup;
use super::button::ButtonState;
use super::button_face::ButtonFace;
use super::defaults::Defaults;
use super::error::Error;
use super::event_handler::EventHandler;
use super::page::Page;
use crate::config;
use std::collections::HashMap;
use std::sync::Arc;
use streamdeck_hid_rs::StreamDeckType;

/// The complete app state!
pub struct AppState {
    /// Defaults!
    defaults: Defaults,
    /// Named buttons, that can be used and modified
    named_buttons: HashMap<String, Arc<ButtonSetup>>,
    /// Pages, that can be loaded
    pages: HashMap<String, Arc<Page>>,
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
        let defaults = Defaults::from_config(&config.defaults)?;

        let mut named_buttons: HashMap<String, Arc<ButtonSetup>> = HashMap::new();

        if let Some(config_buttons) = &config.buttons {
            for button_config in config_buttons {
                if named_buttons.contains_key(&button_config.name) {
                    return Err(Error::DuplicateNamedButton(button_config.name.clone()));
                }
                named_buttons.insert(
                    button_config.name.clone(),
                    Arc::new(
                        ButtonSetup::from_config_with_name(
                            &StreamDeckType::Orig,
                            &button_config,
                            &defaults,
                        )
                        .unwrap(),
                    ),
                );
            }
        }

        let mut pages: HashMap<String, Arc<Page>> = HashMap::new();

        for page_config in &config.pages {
            let (page, more_named_buttons) =
                Page::from_config_with_named_buttons(device_type, &page_config, &defaults)?;
            pages.insert(page_config.name.clone(), Arc::new(page));
            for (name, new_named_button) in more_named_buttons {
                if named_buttons.contains_key(&name) {
                    return Err(Error::DuplicateNamedButton(name));
                }
                named_buttons.insert(name, new_named_button);
            }
        }

        let mut buttons = Vec::new();
        for _ in 0..device_type.total_num_buttons() {
            buttons.push(ButtonState::empty());
        }

        let mut result = AppState {
            defaults,
            named_buttons,
            pages,
            buttons,
            device_type: device_type.clone(),
        };

        if let Some(page_names) = &config.default_pages {
            for page_name in page_names {
                result.load_page(page_name)?;
            }
        }
        Ok(result)
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
    pub fn on_button_pressed(&mut self, button_id: usize) -> Option<Arc<EventHandler>> {
        let button = self.buttons.get_mut(button_id)?;
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
    pub fn on_button_released(&mut self, button_id: usize) -> Option<Arc<EventHandler>> {
        let button = self.buttons.get_mut(button_id)?;
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
    pub fn set_rendered_and_get_rendering_faces(&mut self) -> Vec<(u8, Arc<ButtonFace>)> {
        let mut result = Vec::new();
        for id in 0..self.buttons.len() {
            match self.buttons[id].set_rendered_and_get_face_for_rendering(&self.named_buttons) {
                None => {}
                Some(face) => result.push((id as u8, face.clone())),
            }
        }
        result
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
    fn load_page(&mut self, page_name: &String) -> Result<(), Error> {
        // Find the page
        let page = self
            .pages
            .get(page_name)
            .ok_or(Error::PageNotFound(page_name.clone()))?;

        // Load all the buttons
        for button in &page.buttons {
            self.buttons[button.position.to_button_index(&self.device_type)]
                .set_setup(&button.setup);
        }

        // All went fine!
        Ok(())
    }

    /// React to a foreground window
    pub fn on_foreground_window(
        &mut self,
        title: &String,
        executable: &String,
        class_name: &String,
    ) -> Result<(), Error> {
        let mut pages_to_load = Vec::new();

        for (page_name, page) in &self.pages {
            for condition in &page.on_foreground_window {
                if condition.matches(title, executable, class_name) {
                    pages_to_load.push(page_name.clone());
                }
            }
        }

        for page_name in pages_to_load {
            self.load_page(&page_name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ForegroundWindowConditionConfig;

    /// Returns a full config to be used in tests
    ///
    /// The config contains 1 page with all buttons!
    fn get_full_config(add_doubled_name_error: bool) -> config::Config {
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
                    code: format!("on_named_button{}_up", i),
                }),
                down_handler: Some(config::EventHandlerConfig::AsCode {
                    code: format!("on_named_button{}_down", i),
                }),
            });
        }

        let mut pages = Vec::new();

        for page_id in 0..3 {
            let mut page_buttons = Vec::new();
            for button_id in 0..15 {
                if add_doubled_name_error {}

                page_buttons.push(config::PageButtonConfig {
                    position: config::ButtonPositionConfig {
                        row: button_id / 5,
                        col: button_id % 5,
                    },
                    button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                        name: Some(
                            if add_doubled_name_error && button_id == 0 && page_id == 0 {
                                format!("named_button0")
                            } else {
                                format!("page{}_button{}", page_id, button_id)
                            },
                        ),
                        up_face: Some(config::ButtonFaceConfig {
                            color: None,
                            file: None,
                            label: None,
                            sublabel: None,
                            superlabel: None,
                        }),
                        down_face: None,
                        up_handler: Some(config::EventHandlerConfig::AsCode {
                            code: format!("on_page{}_button{}_up", page_id, button_id),
                        }),
                        down_handler: Some(config::EventHandlerConfig::AsCode {
                            code: format!("on_page{}_button{}_down", page_id, button_id),
                        }),
                    }),
                });
            }
            pages.push(config::PageConfig {
                on_app: Some(vec![ForegroundWindowConditionConfig {
                    executable: Some(format!("{}_exec", page_id)),
                    title: Some(format!("{}_title", page_id)),
                    class_name: None,
                }]),
                name: format!("page{}", page_id),
                buttons: page_buttons,
            });
        }

        let on_app = None;

        config::Config {
            defaults: None,
            buttons: Some(named_buttons),
            pages,
            on_app,
            default_pages: Some(vec!["page0".to_string()]),
        }
    }

    #[test]
    fn named_buttons_must_be_unique() {
        // Setup
        let config = get_full_config(true);

        // Act
        let result = AppState::from_config(&StreamDeckType::Orig, &config);

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn named_buttons_are_loaded_from_config() {
        // Setup
        let config = get_full_config(false);

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
        let config = get_full_config(false);

        // Act
        let state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        //Test
        assert!(state.pages.get(&String::from("page1")).is_some());
    }

    #[test]
    fn named_buttons_of_page_appear_in_named_buttons() {
        // Setup
        let config = get_full_config(false);

        // Act
        let state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        //Test
        for i in 0..15 {
            assert!(state
                .named_buttons
                .get(&format!("page1_button{}", i))
                .is_some());
        }
    }

    #[test]
    fn correct_button_press_and_release_events_are_returned() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        // Page0 is default and loaded!
        let press_event = state.on_button_pressed(0).unwrap();
        let release_event = state.on_button_released(0).unwrap();

        //Test
        assert_eq!(press_event.script, String::from("on_page0_button4_down"));
        assert_eq!(release_event.script, String::from("on_page0_button4_up"));
    }

    #[test]
    fn default_page_is_loaded_and_all_buttons_need_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();

        // Test
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 15);
    }

    #[test]
    fn after_button_press_face_is_returned_for_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 15);
        state.on_button_pressed(0);

        // Test
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 1);
    }

    #[test]
    fn after_button_release_face_is_returned_for_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.on_button_pressed(0);
        state.set_rendered_and_get_rendering_faces();
        state.on_button_released(0);

        // Test
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 1);
    }

    #[test]
    fn button_press_and_release_results_in_no_need_for_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.on_button_pressed(0);
        state.on_button_released(0);

        // Test
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 0);
    }

    #[test]
    fn page_loading_results_in_face_for_new_button_returned_on_button_press() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.load_page(&"page1".to_string()).unwrap();

        // Test
        assert_eq!(state.set_rendered_and_get_rendering_faces().len(), 15);
    }

    #[test]
    fn not_existing_page_loading_results_in_error() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        let result = state.load_page(&String::from("unkown_page"));

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn load_page_on_window_title() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state
            .on_foreground_window(
                &String::from("This is a title for loading page2_title page"),
                &String::from("Some executable we don't care about"),
                &String::from("Some class we don't care about"),
            )
            .unwrap();

        // Test
        assert!(false);
    }

    #[test]
    fn load_page_on_window_executable() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state
            .on_foreground_window(
                &String::from("This is a title for we don't care about"),
                &String::from("/usr/bin/page2_exec"),
                &String::from("Some class we don't care about"),
            )
            .unwrap();

        // Test
        assert!(false);
    }
}
