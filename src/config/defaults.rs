use super::color::ColorConfig;
use serde::Deserialize;

/// Defaults section of the config file.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DefaultsConfigSection {
    background_color: Option<ColorConfig>,
    label_color: Option<ColorConfig>,
    superlabel_color: Option<ColorConfig>,
    sublabel_color: Option<ColorConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::color::ColorConfig::{HEXString, RGB};
    use crate::config::color::ColorConfigRGB;

    #[test]
    fn test_color_all_missing() {
        // Setup
        let yaml = "{}";

        // Act
        let deserialize: DefaultsConfigSection = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.background_color, None);
        assert_eq!(deserialize.label_color, None);
        assert_eq!(deserialize.superlabel_color, None);
        assert_eq!(deserialize.sublabel_color, None);
    }

    #[test]
    fn test_color_all_available() {
        // Setup
        let yaml = "\
background_color: '#FF0000'
label_color:
  red: 0
  green: 1
  blue: 2
superlabel_color: '#00FF00'
sublabel_color: '#FF0000'";

        // Act
        let deserialize: DefaultsConfigSection = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize.background_color,
            Some(HEXString(String::from("#FF0000")))
        );
        assert_eq!(
            deserialize.label_color,
            Some(RGB(ColorConfigRGB {
                red: 0,
                green: 1,
                blue: 2
            }))
        );
        assert_eq!(
            deserialize.superlabel_color,
            Some(HEXString(String::from("#00FF00")))
        );
        assert_eq!(
            deserialize.sublabel_color,
            Some(HEXString(String::from("#FF0000")))
        );
    }
}
