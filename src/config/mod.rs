use serde::Deserialize;

mod button;
mod button_face;
mod button_position;
/// Load configuration file.
///
/// See the (example config)[../../doc/example_config.yml].
mod color;
mod defaults;
mod event_handler;
mod label;
mod page;

/// The complete config for streamdeck-controller-rs
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    defaults: Option<defaults::DefaultsConfigSection>,
    buttons: Option<Vec<button::ButtonConfigWithName>>,
    pages: Vec<page::PageConfig>,
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
