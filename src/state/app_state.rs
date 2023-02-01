use super::button::ButtonSetup;
use super::button::ButtonState;
use super::button_face::ButtonFace;
use super::defaults::Defaults;
use super::error::Error;
use super::event_handler::EventHandler;
use super::page::Page;
use crate::config;
use crate::config::{ButtonConfigWithName, ButtonFaceConfig, ColorConfig};
use crate::foreground_window::WindowInformation;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;
use image::Rgba;
use streamdeck_hid_rs::StreamDeckType;

/// The complete app state!
pub struct AppState {
    /// Defaults!
    defaults: Defaults,
    /// Named buttons, that can be used and modified
    named_buttons: HashMap<String, ButtonSetup>,
    /// Pages, that can be loaded
    pages: HashMap<String, Arc<Page>>,
    /// The current loaded buttons
    buttons: Vec<ButtonState>,
    /// The current stack of loaded pages
    loaded_pages: Vec<String>,
    /// The device type this is for!
    device_type: StreamDeckType,
    /// Init event handler
    init_handler: Option<Arc<EventHandler>>,
    /// The current foreground window
    foreground_window: Option<WindowInformation>,
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

        let mut named_buttons: HashMap<String, ButtonSetup> = HashMap::new();

        if let Some(config_buttons) = &config.buttons {
            for button_config in config_buttons {
                if named_buttons.contains_key(&button_config.name) {
                    return Err(Error::DuplicateNamedButton(button_config.name.clone()));
                }
                named_buttons.insert(
                    button_config.name.clone(),
                        ButtonSetup::from_config_with_name(&device_type, &button_config, &defaults)
                            .unwrap(),
                );
            }
        }

        // Create a special empty named button (that can be overwritten)
        if !named_buttons.contains_key("empty") {
            named_buttons.insert(
                "empty".to_string(),
                ButtonSetup::from_config_with_name(
                    &device_type,
                    &ButtonConfigWithName {
                        name: "empty".to_string(),
                        up_face: Some(ButtonFaceConfig {
                            color: Some(ColorConfig::HEXString("#000000".to_string())),
                            file: None,
                            label: None,
                            sublabel: None,
                            superlabel: None,
                        }),
                        down_face: None,
                        up_handler: None,
                        down_handler: None,
                    },
                    &defaults,
                )
                    .unwrap(),
            );
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

        let init_handler = if let Some(init_event_config) = &config.init_script {
            Some(Arc::new(EventHandler::from_config(&init_event_config)?))
        } else {
            None
        };

        let mut result = AppState {
            defaults,
            named_buttons,
            pages,
            buttons,
            init_handler,
            device_type: device_type.clone(),
            loaded_pages: Vec::new(),
            foreground_window: None,
        };

        if let Some(page_names) = &config.default_pages {
            for page_name in page_names {
                result.load_page(page_name)?;
            }
        }
        Ok(result)
    }

    /// Returns the init event to be executed by the script engine
    pub fn get_init_handler(&self) -> Option<Arc<EventHandler>> {
        self.init_handler.clone()
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
    pub fn on_button_pressed(&mut self, button_id: usize) -> Option<&EventHandler> {
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
    pub fn on_button_released(&mut self, button_id: usize) -> Option<&EventHandler> {
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
    pub fn set_rendered_and_get_rendering_faces(&mut self) -> Vec<(u8, &ButtonFace)> {
        let mut result = Vec::new();
        for (id, button) in self.buttons.iter_mut().enumerate() {
            match button.set_rendered_and_get_face_for_rendering(&self.named_buttons) {
                None => {}
                Some(face) => result.push((id as u8, face.clone())),
            }
        }
        result
    }

    /// Updates the up face of a named button.
    ///
    /// # Arguments
    ///
    /// button_name - The name of the named button
    ///
    /// # Return
    ///
    /// () if all went ok, Error if the button was ot found.
    pub fn set_named_button_up_face(
        &mut self,
        button_name: &String,
        color: Option<Rgba<u8>>,
        file: Option<String>,
        label: Option<String>,
        labelcolor: Option<Rgba<u8>>,
        sublabel: Option<String>,
        sublabelcolor: Option<Rgba<u8>>,
        superlabel: Option<String>,
        superlabelcolor: Option<Rgba<u8>>,
    ) -> Result<(), Error> {
        // Find the button
        let mut button= self
            .named_buttons
            .get_mut(button_name)
            .ok_or(Error::ButtonNotFound(button_name.clone()))?;

        // Update the button
        if let Some(uf) = &mut button.up_face {
            uf.update_values(color, file, label, labelcolor, sublabel, sublabelcolor, superlabel, superlabelcolor, &self.defaults)?;
        } else {
            let mut uf = ButtonFace::empty(self.device_type.clone());
            uf.update_values(color, file, label, labelcolor, sublabel, sublabelcolor, superlabel, superlabelcolor, &self.defaults)?;
            button.up_face = Some(uf);
        }
        // Set all buttons using this to re-render!
        for mut button in self.buttons.iter_mut() {
            if button.uses_button(button_name) {
                button.set_needs_rendering();
            }
        }

        Ok(())
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
    pub fn load_page(&mut self, page_name: &String) -> Result<(), Error> {
        // Find the page
        let page = self
            .pages
            .get(page_name)
            .ok_or(Error::PageNotFound(page_name.clone()))?;

        // Add page to stack
        self.loaded_pages.push(page_name.clone());

        // Load all the buttons
        for button in &page.buttons {
            self.buttons[button.position.to_button_index(&self.device_type)]
                .set_button(button.button_name.clone());
        }

        // All went fine!
        debug!("page {} loaded", page_name);
        Ok(())
    }

    /// Unloads a page, setting all the buttons that originate from this page to be empty.
    ///
    /// # Arguments
    ///
    /// page_name - Name of the page to be un-loaded.
    ///
    /// # Return
    ///
    /// () if all went ok, Error if something went wrong
    pub fn unload_page(&mut self, page_name: &String) -> Result<(), Error> {
        // Find the page
        let page = self
            .pages
            .get(page_name)
            .ok_or(Error::PageNotFound(page_name.clone()))?;

        // Remove the page from the stack
        self.loaded_pages.retain(|i| i != page_name);

        // Get through all the buttons
        for button_index in 0..self.device_type.total_num_buttons() {
            if page.get_button(&self.device_type, button_index).is_some() {
                // Button needs to be removed, that means we have to find the correct button from the stack!
                self.buttons[button_index].set_button("empty".to_string());
                for stack_page_name in &self.loaded_pages {
                    if let Some(button) = self
                        .pages
                        .get(stack_page_name.as_str())
                        .and_then(|p| p.get_button(&self.device_type, button_index))
                    {
                        self.buttons[button_index].set_button(button.button_name.clone());
                    }
                }
            }
        }

        // All went fine!
        debug!("page {} un-loaded", page_name);
        Ok(())
    }

    /// React to a foreground window
    pub fn on_foreground_window(&mut self, window_info: &WindowInformation) -> Result<(), Error> {
        let mut pages_to_load = Vec::new();
        let mut pages_to_unload: Vec<String> = Vec::new();

        for (page_name, page) in &self.pages {
            for condition in &page.on_foreground_window {
                if condition.matches(window_info) {
                    pages_to_load.push(page_name.clone());
                } else if page.unload_if_not_loaded && self.loaded_pages.contains(page_name) {
                    pages_to_unload.push(page_name.clone());
                }
            }
        }

        self.foreground_window = Some(window_info.clone());

        for page_name in pages_to_load {
            self.load_page(&page_name)?;
        }

        for page_name in pages_to_unload {
            self.unload_page(&page_name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ForegroundWindowConditionConfig, PageLoadConditions};
    use image::RgbImage;
    use std::borrow::Borrow;
    use std::collections::hash_map::RandomState;
    use std::collections::HashSet;

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
                    position: config::ButtonPositionConfig::ButtonPositionObjectConfig(
                        config::ButtonPositionObject {
                            row: button_id / 5,
                            col: button_id % 5,
                        },
                    ),
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
                            label: Some(config::LabelConfig::JustText(format!(
                                "page{}_button{}",
                                page_id, button_id
                            ))),
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
                on_app: Some(PageLoadConditions {
                    conditions: vec![ForegroundWindowConditionConfig {
                        executable: Some(format!(".*page{}_exec.*", page_id)),
                        title: Some(format!(".*page{}_title.*", page_id)),
                        class_name: None,
                    }],
                    remove: None,
                }),
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
            init_script: None,
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

    // Get the md5 sum of an image
    fn image_md5(i: &RgbImage) -> md5::Digest {
        md5::compute(i.as_raw())
    }

    #[test]
    fn page_loading_results_in_face_for_new_button_returned_for_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.load_page(&"page1".to_string()).unwrap();

        // Test
        let face_md5s = HashSet::<md5::Digest, RandomState>::from_iter(
            state
                .set_rendered_and_get_rendering_faces()
                .iter()
                .map(|f| image_md5(&f.1.face)),
        );
        assert_eq!(face_md5s.len(), 15);
        for index in 0..15 {
            assert!(face_md5s.contains(&image_md5(
                &state
                    .named_buttons
                    .get(format!("page1_button{index}").as_str())
                    .unwrap()
                    .up_face
                    .as_ref()
                    .unwrap()
                    .face
            )))
        }
    }

    #[test]
    fn page_loading_and_unloading_results_in_face_for_empty_needing_rendering() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.unload_page(&"page0".to_string()).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.load_page(&"page1".to_string()).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.unload_page(&"page1".to_string()).unwrap();

        // Test
        let rendering_faces = state.set_rendered_and_get_rendering_faces();
        let face_md5s = HashSet::<md5::Digest, RandomState>::from_iter(
            rendering_faces.iter().map(|f| image_md5(&f.1.face)),
        );
        assert_eq!(rendering_faces.len(), 15);
        assert_eq!(face_md5s.len(), 1);
        assert!(face_md5s.contains(&image_md5(
            &state
                .named_buttons
                .get("empty".to_string().as_str())
                .unwrap()
                .up_face
                .as_ref()
                .unwrap()
                .face
        )));
    }

    #[test]
    fn page_unloading_over_other_page_results_in_other_page_visible() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.load_page(&"page1".to_string()).unwrap();
        state.load_page(&"page2".to_string()).unwrap();
        state.set_rendered_and_get_rendering_faces();
        state.unload_page(&"page2".to_string()).unwrap();

        // Test
        let rendering_faces = state.set_rendered_and_get_rendering_faces();
        let face_md5s = HashSet::<md5::Digest, RandomState>::from_iter(
            rendering_faces.iter().map(|f| image_md5(&f.1.face)),
        );
        assert_eq!(rendering_faces.len(), 15);
        assert_eq!(face_md5s.len(), 15);
        for index in 0..15 {
            assert!(face_md5s.contains(&image_md5(
                &state
                    .named_buttons
                    .get(format!("page1_button{index}").as_str())
                    .unwrap()
                    .up_face
                    .as_ref()
                    .unwrap()
                    .face
            )))
        }
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
    fn load_page_on_window() {
        // Setup
        let config = get_full_config(false);

        // Act
        let mut state = AppState::from_config(&StreamDeckType::Orig, &config).unwrap();
        state
            .on_foreground_window(&WindowInformation {
                title: String::from("This is a title for loading page2_title page"),
                executable: String::from("/usr/bin/page2_exec"),
                class_name: String::from("Some class we don't care about"),
            })
            .unwrap();

        // Test
        assert_eq!(
            state.on_button_pressed(0).unwrap().script,
            "on_page2_button4_down"
        );
    }
}
