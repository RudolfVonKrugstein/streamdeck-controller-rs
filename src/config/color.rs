use crate::config::error;
use serde::Deserialize;

/// Color in the configuration.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum ColorConfig {
    /// The color, when it is provided as an HEX string (example #FF0000)
    HEXString(String),
    /// The color with explicit values for red, green and blue
    RGB(ColorConfigRGB),
}

pub fn hex_string_to_rgba_color(hex: &String) -> Result<image::Rgba<u8>, error::Error> {
    if &hex[..1] != "#" {
        return Err(error::Error::InvalidColorHexString(hex.clone()));
    }
    let without_prefix = hex.trim_start_matches("#");
    let num = u32::from_str_radix(without_prefix, 16)
        .map_err(|_| error::Error::InvalidColorHexString(hex.clone()))?;
    // Result
    match without_prefix.len() {
        6 => Ok(image::Rgba([
            (num >> 16) as u8,
            (num >> 8) as u8,
            (num & 0xFF) as u8,
            255,
        ])),
        8 => Ok(image::Rgba([
            (num >> 24) as u8,
            (num >> 16) as u8,
            (num >> 8) as u8,
            (num & 0xFF) as u8,
        ])),
        _ => Err(error::Error::InvalidColorHexString(hex.clone())),
    }
}

impl ColorConfig {
    /// Convert to an image color.
    pub fn to_image_rgba_color(&self) -> Result<image::Rgba<u8>, error::Error> {
        match self {
            ColorConfig::HEXString(hex) => hex_string_to_rgba_color(hex),
            ColorConfig::RGB(c) => Ok(image::Rgba([c.red, c.green, c.blue, 0xFF])),
        }
    }
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

    #[test]
    fn hex_to_rgba() {
        // Setup
        let hex_color = ColorConfig::HEXString(String::from("#000FFF"));

        // Act
        let color = hex_color.to_image_rgba_color().unwrap();

        // Test
        assert_eq!(color.0[0], 0);
        assert_eq!(color.0[1], 0x0F);
        assert_eq!(color.0[2], 0xFF);
        assert_eq!(color.0[3], 0xFF);
    }

    #[test]
    fn hex_with_alpha_to_rgba() {
        // Setup
        let hex_color = ColorConfig::HEXString(String::from("#000FFFF0"));

        // Act
        let color = hex_color.to_image_rgba_color().unwrap();

        // Test
        assert_eq!(color.0[0], 0);
        assert_eq!(color.0[1], 0x0F);
        assert_eq!(color.0[2], 0xFF);
        assert_eq!(color.0[3], 0xF0);
    }

    #[test]
    fn invalid_hex_string() {
        // Setup
        let hex_color = ColorConfig::HEXString(String::from("000FFF"));

        // Act
        let result = hex_color.to_image_rgba_color();

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn invalid_length_hex_string() {
        // Setup
        let hex_color = ColorConfig::HEXString(String::from("#000FFF1"));

        // Act
        let result = hex_color.to_image_rgba_color();

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn non_hex_to_rgba() {
        // Setup
        let hex_color = ColorConfig::RGB(ColorConfigRGB {
            red: 1,
            green: 2,
            blue: 3,
        });

        // Act
        let color = hex_color.to_image_rgba_color().unwrap();

        // Test
        assert_eq!(color.0[0], 1);
        assert_eq!(color.0[1], 2);
        assert_eq!(color.0[2], 3);
        assert_eq!(color.0[3], 0xFF);
    }
}
