use super::super::button_position::ButtonPosition;
use crate::config;
use crate::state::button::ButtonSetup;
use crate::state::defaults::Defaults;
use crate::state::error::Error;
use std::sync::Arc;
use log::warn;
use pyo3::number::pos;
use streamdeck_hid_rs::StreamDeckType;
use crate::config::ButtonOrButtonName;

/// Setup of a button with position!
pub struct PositionedButtonSetup {
    pub position: ButtonPosition,
    pub button_name: String,
}

impl PositionedButtonSetup {
    /// Create the PositionedButtonSetup from the configuration.
    ///
    /// As a side effect, this might also create a named button (if the button is given a config
    /// in the button itself the config this creates a named button).
    ///
    /// # Arguments
    ///
    /// The config to create the object from.
    ///
    /// # Result
    ///
    /// On success the Results contains a tuple with the [PositionedButtonSetup] itself.
    /// If it is a named button, the named button to be created is returned as a tuple
    /// of the name and the button setup..
    pub fn from_config_with_named_button(
        page_name: &String,
        device_type: &StreamDeckType,
        config: &config::PageButtonConfig,
        defaults: &Defaults,
    ) -> Result<(PositionedButtonSetup, Option<(String, ButtonSetup)>), Error> {
        let position = ButtonPosition::from_config(&config.position)?;
        // Create a button or just a name
        match &config.button {
            ButtonOrButtonName::ButtonName(button_name) => {
                Ok((PositionedButtonSetup { position, button_name: button_name.clone() }, None))
            },
            ButtonOrButtonName::Button(setup) => {
                // Set the name
                let button_name = setup.name.clone().unwrap_or_else(|| format!("page_{}_button_{}", page_name, position.to_button_index(device_type)));
                Ok(
                    (
                        PositionedButtonSetup { position, button_name: button_name.clone() },
                        Some(
                            (button_name, ButtonSetup::from_optional_name_config(device_type, setup, defaults)?)
                        )
                    )
                )
            }
        }
        // let (setup, named_button) = ButtonSetupOrName::from_config_with_named_button(
        //     device_type,
        //     &config.button,
        //     defaults,
        // )?;

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ButtonPositionObject;

    #[test]
    fn test_from_config_with_named_button() {
        // Setup
        let config = config::PageButtonConfig {
            position: config::ButtonPositionConfig::ButtonPositionObjectConfig(
                ButtonPositionObject { row: 0, col: 0 },
            ),
            button: config::ButtonOrButtonName::ButtonName(String::from("test_button")),
        };

        // Act
        let _object = PositionedButtonSetup::from_config_with_named_button(
            &"test_page".to_string(),
            &StreamDeckType::Orig,
            &config,
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        assert!(true); // We just assert, that we did not panic!
    }
}
