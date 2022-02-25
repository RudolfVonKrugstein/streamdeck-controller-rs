use serde::Deserialize;

mod button;
pub use button::*;
mod button_face;
pub use button_face::*;
mod button_position;
/// Load configuration file.
///
/// See the (example config)[../../doc/example_config.yml].
mod color;
pub use color::*;
mod defaults;
mod event_handler;
pub use event_handler::*;
mod label;
pub use label::*;
mod error;
mod page;
pub use error::*;

/// The complete config for streamdeck-controller-rs
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub defaults: Option<defaults::DefaultsConfigSection>,
    pub buttons: Option<Vec<button::ButtonConfigWithName>>,
    pub pages: Vec<page::PageConfig>,
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
}
