use super::error::Error;
use crate::config;

/// Defaults, that fill missing values
#[derive(Debug)]
pub struct Defaults {
    pub background_color: image::Rgba<u8>,
    pub label_color: image::Rgba<u8>,
    pub superlabel_color: image::Rgba<u8>,
    pub sublabel_color: image::Rgba<u8>,
}

impl Defaults {
    pub fn from_config(config: &Option<config::DefaultsConfig>) -> Result<Defaults, Error> {
        let mut background_color = image::Rgba([0, 0, 0, 255]);
        let mut label_color = image::Rgba([255, 255, 255, 255]);
        let mut superlabel_color = image::Rgba([255, 255, 0, 255]);
        let mut sublabel_color = image::Rgba([0, 255, 255, 255]);

        if let Some(config) = config {
            background_color = match &config.background_color {
                None => background_color,
                Some(c) => c.to_image_rgba_color().map_err(Error::ConfigError)?,
            };
            label_color = match &config.label_color {
                None => label_color,
                Some(c) => c.to_image_rgba_color().map_err(Error::ConfigError)?,
            };
            superlabel_color = match &config.superlabel_color {
                None => superlabel_color,
                Some(c) => c.to_image_rgba_color().map_err(Error::ConfigError)?,
            };
            sublabel_color = match &config.sublabel_color {
                None => sublabel_color,
                Some(c) => c.to_image_rgba_color().map_err(Error::ConfigError)?,
            };
        }

        Ok(Defaults {
            background_color,
            superlabel_color,
            sublabel_color,
            label_color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_defaults() {
        // Setup
        let config = Some(config::DefaultsConfig {
            background_color: None,
            label_color: None,
            superlabel_color: None,
            sublabel_color: None,
        });

        // Act
        let defaults = Defaults::from_config(&config).unwrap();
        let defaults_from_none = Defaults::from_config(&None).unwrap();

        // Test
        assert_eq!(defaults.background_color, image::Rgba([0, 0, 0, 255]));
        assert_eq!(defaults.label_color, image::Rgba([255, 255, 255, 255]));
        assert_eq!(defaults.superlabel_color, image::Rgba([255, 255, 0, 255]));
        assert_eq!(defaults.sublabel_color, image::Rgba([0, 255, 255, 255]));

        assert_eq!(
            defaults_from_none.background_color,
            image::Rgba([0, 0, 0, 255])
        );
        assert_eq!(
            defaults_from_none.label_color,
            image::Rgba([255, 255, 255, 255])
        );
        assert_eq!(
            defaults_from_none.superlabel_color,
            image::Rgba([255, 255, 0, 255])
        );
        assert_eq!(
            defaults_from_none.sublabel_color,
            image::Rgba([0, 255, 255, 255])
        );
    }
}
