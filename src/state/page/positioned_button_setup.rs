use super::super::button::ButtonSetupOrName;
use super::super::button_position::ButtonPosition;
use crate::config;
use crate::state::button::ButtonSetup;
use crate::state::defaults::Defaults;
use crate::state::error::Error;
use std::sync::Arc;
use streamdeck_hid_rs::StreamDeckType;

/// Setup of a button with position!
pub struct PositionedButtonSetup {
    pub position: ButtonPosition,
    pub setup: ButtonSetupOrName,
}

impl PositionedButtonSetup {
    /// Create the PositionedButtonSetup from the configuration.
    ///
    /// As a side effect, this might also create a named button (if the button is given a name in
    /// the config this creates a named button).
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
        device_type: &StreamDeckType,
        config: &config::PageButtonConfig,
        defaults: &Defaults,
    ) -> Result<(PositionedButtonSetup, Option<(String, Arc<ButtonSetup>)>), Error> {
        let (setup, named_button) = ButtonSetupOrName::from_config_with_named_button(
            device_type,
            &config.button,
            defaults,
        )?;
        let position = ButtonPosition::from_config(&config.position);

        Ok((PositionedButtonSetup { position, setup }, named_button))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config_with_named_button() {
        // Setup
        let config = config::PageButtonConfig {
            position: config::ButtonPositionConfig { row: 0, col: 0 },
            button: config::ButtonOrButtonName::ButtonName(String::from("test_button")),
        };

        // Act
        let _object = PositionedButtonSetup::from_config_with_named_button(
            &StreamDeckType::Orig,
            &config,
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        assert!(true); // We just assert, that we did not panic!
    }
}
