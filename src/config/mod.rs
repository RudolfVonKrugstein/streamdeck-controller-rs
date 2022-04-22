use serde::Deserialize;

mod button;
pub use button::*;
mod button_face;
pub use button_face::*;
mod button_position;
pub use button_position::*;
/// Load configuration file.
///
/// See the (example config)[../../doc/example_config.yml].
mod color;
pub use color::*;
mod defaults;
pub use defaults::*;
mod event_handler;
pub use event_handler::*;
mod label;
pub use label::*;
mod error;
pub use error::*;
mod foreground_window_condition;
mod foreground_window_handler;
mod page;

pub use foreground_window_condition::*;

use crate::config::foreground_window_handler::ForegroundWindowHandlerConfig;
pub use page::*;

/// The complete config for streamdeck-controller-rs
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub defaults: Option<defaults::DefaultsConfig>,
    pub buttons: Option<Vec<button::ButtonConfigWithName>>,
    pub pages: Vec<page::PageConfig>,
    pub default_pages: Option<Vec<String>>,
    pub init_script: Option<EventHandlerConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_default_config() {
        // Setup
        let yaml = include_str!("../../doc/example_config.yml");

        // Act
        let result: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_ok());
    }

    #[test]
    fn fail_on_config_with_unkown_fields() {
        // Setup
        let yaml = "not_allowed: {}";

        // Act
        let result: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_err());
    }
}
