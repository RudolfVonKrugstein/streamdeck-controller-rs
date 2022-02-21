use serde::Deserialize;

/// Color in the configuration.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ColorConfig {
    /// The color, when it is provided as an HEX string (example #FF0000)
    HEXString(String),
    /// The color with explicit values for red, green and blue
    RGB(ColorConfigRGB),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ColorConfigRGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex_string() {
        // Setup
        let hex_value = "#FF0000";
        let yaml = format!("'{}'", hex_value);

        // Act
        let deserialize: ColorConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize, ColorConfig::HEXString(String::from(hex_value)));
    }

    #[test]
    fn test_color_from_rgb() {
        // Setup
        let red: u8 = 0;
        let green: u8 = 1;
        let blue: u8 = 2;
        let yaml = format!("red: {}\ngreen: {}\nblue: {}", red, green, blue);

        // Act
        let deserialize: ColorConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            ColorConfig::RGB(ColorConfigRGB { red, green, blue })
        );
    }
}
