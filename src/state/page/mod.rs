mod positioned_button_setup;

use positioned_button_setup::*;

use super::error::Error;
use crate::config;
use crate::state::button::ButtonSetup;
use crate::state::defaults::Defaults;
use crate::state::foreground_window_condition::ForegroundWindowCondition;
use std::collections::HashMap;
use std::sync::Arc;
use streamdeck_hid_rs::StreamDeckType;

/// A page, that can be loaded!
pub struct Page {
    pub buttons: Vec<PositionedButtonSetup>,
    pub on_foreground_window: Vec<ForegroundWindowCondition>,
    pub unload_if_not_loaded: bool,
}

impl Page {
    /// Creates the page from config, also returns a list of named buttons that
    /// have been created for this page!
    pub fn from_config_with_named_buttons(
        device_type: &StreamDeckType,
        config: &config::PageConfig,
        defaults: &Defaults,
    ) -> Result<(Page, HashMap<String, Arc<ButtonSetup>>), Error> {
        let mut buttons = Vec::new();
        let mut named_buttons = HashMap::new();
        let mut unload_if_not_loaded = false;
        let on_foreground_window = match &config.on_app {
            None => Vec::new(),
            Some(configs) => {
                let mut l = Vec::new();
                unload_if_not_loaded = configs.remove == Some(true);
                for c in &configs.conditions {
                    l.push(ForegroundWindowCondition::from_config(c)?);
                }
                l
            }
        };

        for button_config in &config.buttons {
            let (button, named_button) = PositionedButtonSetup::from_config_with_named_button(
                &config.name,
                device_type,
                button_config,
                defaults,
            )?;
            buttons.push(button);
            if let Some((name, named_button)) = named_button {
                named_buttons.insert(name, named_button);
            }
        }

        Ok((
            Page {
                on_foreground_window,
                buttons,
                unload_if_not_loaded,
            },
            named_buttons,
        ))
    }

    /// Get button at position, if it exists
    pub fn get_button(
        &self,
        device_type: &StreamDeckType,
        button_index: usize,
    ) -> Option<&PositionedButtonSetup> {
        for button in &self.buttons {
            if button.position.to_button_index(device_type) == button_index {
                return Some(button);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use crate::config::ButtonPositionObject;

    #[test]
    fn no_buttons_with_names_no_named_buttons() {
        // Setup
        let config = config::PageConfig {
            name: String::from("page1"),
            on_app: None,
            buttons: Vec::from([
                config::PageButtonConfig {
                    position: config::ButtonPositionConfig::ButtonPositionObjectConfig(
                        ButtonPositionObject { row: 0, col: 0 },
                    ),
                    button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                        name: None,
                        up_face: None,
                        down_face: None,
                        up_handler: None,
                        down_handler: None,
                    }),
                },
                config::PageButtonConfig {
                    position: config::ButtonPositionConfig::ButtonPositionObjectConfig(
                        ButtonPositionObject { row: 0, col: 1 },
                    ),
                    button: config::ButtonOrButtonName::ButtonName(String::from("named_button")),
                },
            ]),
        };
        let defaults = Defaults::from_config(&None).unwrap();

        // Act
        let (page, named_buttons) =
            Page::from_config_with_named_buttons(&StreamDeckType::Orig, &config, &defaults)
                .unwrap();

        // Result
        assert!(named_buttons.is_empty());
        assert_eq!(page.buttons.len(), 2);
    }

    #[test]
    fn buttons_with_names_produce_named_buttons() {
        // Setup
        let config = config::PageConfig {
            name: String::from("page1"),
            on_app: None,
            buttons: Vec::from([config::PageButtonConfig {
                position: config::ButtonPositionConfig::ButtonPositionObjectConfig(
                    ButtonPositionObject { row: 0, col: 0 },
                ),
                button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                    name: Some(String::from("button_name")),
                    up_face: None,
                    down_face: None,
                    up_handler: None,
                    down_handler: None,
                }),
            }]),
        };
        let defaults = Defaults::from_config(&None).unwrap();

        // Act
        let (page, named_buttons) =
            Page::from_config_with_named_buttons(&StreamDeckType::Orig, &config, &defaults)
                .unwrap();

        // Result
        assert_eq!(named_buttons.len(), 1);
        assert_eq!(page.buttons.len(), 1);
    }
}
