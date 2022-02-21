use serde::{Deserialize};
use crate::config::color::ColorConfig;
use crate::config::label::LabelConfig;


/// The face of a button (what is displayed on a button) from the config.
#[derive(Debug, Deserialize, PartialEq)]
pub struct ButtonFaceConfig {
    pub color: Option<ColorConfig>,
    pub file: Option<String>,
    pub label: Option<LabelConfig>,
    pub sublabel: Option<LabelConfig>,
    pub superlabel: Option<LabelConfig>
}

#[cfg(test)]
mod tests {
    use crate::config::label::LabelConfigWithColor;
    use super::*;

    #[test]
    fn test_without_anything() {
        // Setup
        let yaml = "{}";

        // Act
        let deserialize: ButtonFaceConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.color, None);
        assert_eq!(deserialize.label, None);
        assert_eq!(deserialize.sublabel, None);
        assert_eq!(deserialize.superlabel, None);
    }

    #[test]
    fn fails_with_missing_text() {
        // Setup
        let yaml = "label: {}";

        // Act
        let deserialize: Result<ButtonFaceConfig, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert_eq!(deserialize.is_err(), true);
    }

    #[test]
    fn test_with_all_values() {
        // Setup
        let color_value = "#FF0000";
        let file_value = "image.png";
        let label_value = "label";
        let label_color_value = "#F00000";
        let sub_label_value = "label";
        let sub_label_color_value = "#0F0000";
        let super_label_value = "label";
        let super_label_color_value = "#00F000";
        let yaml = format!("\
color: '{}'
file: {}
label:
  text: {}
  color: '{}'
sublabel:
  text: {}
  color: '{}'
superlabel:
  text: {}
  color: '{}'",
                           color_value, file_value,
                           label_value, label_color_value,
                           sub_label_value, sub_label_color_value,
                           super_label_value, super_label_color_value);

        // Act
        let deserialize: ButtonFaceConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.color, Some(ColorConfig::HEXString(String::from(color_value))));
        assert_eq!(deserialize.file, Some(String::from(file_value)));
        assert_eq!(deserialize.label, Some(LabelConfig::WithColor(LabelConfigWithColor{
            text: String::from(label_value),
            color: Some(ColorConfig::HEXString(String::from(label_color_value)))
        })));
        assert_eq!(deserialize.sublabel, Some(LabelConfig::WithColor(LabelConfigWithColor{
            text: String::from(sub_label_value),
            color: Some(ColorConfig::HEXString(String::from(sub_label_color_value)))
        })));
        assert_eq!(deserialize.superlabel, Some(LabelConfig::WithColor(LabelConfigWithColor {
            text: String::from(super_label_value),
            color: Some(ColorConfig::HEXString(String::from(super_label_color_value)))
        })));
    }
}

