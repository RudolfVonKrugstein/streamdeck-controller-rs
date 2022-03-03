mod positioned_button_setup;
use positioned_button_setup::*;

use super::error::Error;
use crate::config;
use crate::state::button::ButtonSetup;
use crate::state::foreground_window_condition::ForegroundWindowCondition;
use std::collections::HashMap;
use std::sync::Arc;
use streamdeck_hid_rs::StreamDeckType;

/// A page, that can be loaded!
pub struct Page {
    pub buttons: Vec<PositionedButtonSetup>,
    pub on_foreground_window: Vec<ForegroundWindowCondition>,
}

impl Page {
    /// Creates the page from config, also returns a list of named buttons that
    /// have been created for this page!
    pub fn from_config_with_named_buttons(
        device_type: &StreamDeckType,
        config: &config::PageConfig,
    ) -> Result<(Page, HashMap<String, Arc<ButtonSetup>>), Error> {
        let mut buttons = Vec::new();
        let mut named_buttons = HashMap::new();
        let on_foreground_window = match &config.on_app {
            None => Vec::new(),
            Some(configs) => {
                let mut l = Vec::new();
                for c in configs {
                    l.push(ForegroundWindowCondition::from_config(&c)?);
                }
                l
            }
        };

        for button_config in &config.buttons {
            let (button, named_button) =
                PositionedButtonSetup::from_config_with_named_button(device_type, button_config)?;
            buttons.push(button);
            if let Some((name, named_button)) = named_button {
                named_buttons.insert(name, named_button);
            }
        }

        Ok((
            Page {
                on_foreground_window,
                buttons,
            },
            named_buttons,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn no_buttons_with_names_no_named_buttons() {
        // Setup
        let config = config::PageConfig {
            name: String::from("page1"),
            on_app: None,
            buttons: Vec::from([
                config::PageButtonConfig {
                    position: config::ButtonPositionConfig { row: 0, col: 0 },
                    button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                        name: None,
                        up_face: None,
                        down_face: None,
                        up_handler: None,
                        down_handler: None,
                    }),
                },
                config::PageButtonConfig {
                    position: config::ButtonPositionConfig { row: 0, col: 1 },
                    button: config::ButtonOrButtonName::ButtonName(String::from("named_button")),
                },
            ]),
        };

        // Act
        let (page, named_buttons) =
            Page::from_config_with_named_buttons(&StreamDeckType::Orig, &config).unwrap();

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
                position: config::ButtonPositionConfig { row: 0, col: 0 },
                button: config::ButtonOrButtonName::Button(config::ButtonConfigOptionalName {
                    name: Some(String::from("button_name")),
                    up_face: None,
                    down_face: None,
                    up_handler: None,
                    down_handler: None,
                }),
            }]),
        };

        // Act
        let (page, named_buttons) =
            Page::from_config_with_named_buttons(&StreamDeckType::Orig, &config).unwrap();

        // Result
        assert_eq!(named_buttons.len(), 1);
        assert_eq!(page.buttons.len(), 1);
    }
}
