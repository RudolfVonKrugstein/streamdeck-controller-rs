use crate::config::color::ColorConfig;
use serde::Deserialize;

/// A label that can be placed on a button.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum LabelConfig {
    JustText(String),
    WithColor(LabelConfigWithColor),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LabelConfigWithColor {
    pub color: Option<ColorConfig>,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_just_text() {
        // Setup
        let label_value = "label";
        let yaml = format!("'{}'", label_value);

        // Act
        let deserialize: LabelConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            LabelConfig::JustText(String::from(label_value))
        );
    }

    #[test]
    fn test_without_color() {
        // Setup
        let label_value = "label";
        let yaml = format!("text: {}", label_value);

        // Act
        let deserialize: LabelConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            LabelConfig::WithColor(LabelConfigWithColor {
                color: None,
                text: String::from(label_value)
            })
        );
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
        assert_eq!(
            deserialize,
            LabelConfig::WithColor(LabelConfigWithColor {
                color: Some(ColorConfig::HEXString(String::from(color_value))),
                text: String::from(label_value)
            })
        );
    }
}
