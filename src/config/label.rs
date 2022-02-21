use serde::{Deserialize};
use crate::config::color::ColorConfig;


/// A label that can be placed on a button.
#[derive(Debug, Deserialize, PartialEq)]
pub struct LabelConfig {
    pub color: Option<ColorConfig>,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without_color() {
        // Setup
        let label_value = "label";
        let yaml = format!("text: {}", label_value);

        // Act
        let deserialize: LabelConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.color, None);
        assert_eq!(deserialize.text, String::from(label_value));
    }

    #[test]
    fn test_with_color() {
        // Setup
        let label_value = "label";
        let color_value = "#FF0000";
        let yaml = format!("text: {}\ncolor: '{}'", label_value, color_value);

        // Act
        let deserialize: LabelConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.color, Some(ColorConfig::HEXString(String::from(color_value))));
        assert_eq!(deserialize.text, String::from(label_value));
    }
}
