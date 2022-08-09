use super::error::Error;
use super::Defaults;
use crate::config;
use crate::config::LabelConfig;
use image::{Pixel, Rgba};

/// Colored text, used in the button face
struct ColoredText {
    color: Option<Rgba<u8>>,
    text: String,
}

/// Face (picture) to be printed on a button.
///
/// The face is pre-rendered into an image.
pub struct ButtonFace {
    device_type: streamdeck_hid_rs::StreamDeckType,
    pub face: image::RgbImage,
    color: Option<Rgba<u8>>,
    file: Option<String>,
    label: Option<ColoredText>,
    sublabel: Option<ColoredText>,
    superlabel: Option<ColoredText>,
}

impl ButtonFace {
    /// Create a [ButtonFace] from the corresponding
    /// stuff in the configuration ([ButtonFaceConfig]).
    pub fn from_config(
        device_type: &streamdeck_hid_rs::StreamDeckType,
        face_config: &config::ButtonFaceConfig,
        defaults: &Defaults,
    ) -> Result<ButtonFace, Error> {
        let face = image::RgbImage::new(0, 0);
        let mut button = ButtonFace {
            face,
            color: match &face_config.color {
                None => None,
                Some(c) => Some(c.to_image_rgba_color().map_err(Error::ConfigError)?),
            },
            file: face_config.file.clone(),
            label: match &face_config.label {
                None => None,
                Some(label_config) => Some(ColoredText::from_config(label_config)?),
            },
            sublabel: match &face_config.sublabel {
                None => None,
                Some(label_config) => Some(ColoredText::from_config(label_config)?),
            },
            device_type: device_type.clone(),
            superlabel: match &face_config.superlabel {
                None => None,
                Some(label_config) => Some(ColoredText::from_config(label_config)?),
            },
        };
        button.draw_face(defaults)?;
        Ok(button)
    }

    pub fn empty(device_type: streamdeck_hid_rs::StreamDeckType) -> ButtonFace {
        ButtonFace {
            device_type,
            face: image::RgbImage::new(0, 0),
            color: None,
            file: None,
            label: None,
            sublabel: None,
            superlabel: None
        }
    }

    /// Updates the face with new values
    pub fn update_values(&mut self,
                  color: Option<Rgba<u8>>,
                  file: Option<String>,
                  label: Option<String>,
                  labelcolor: Option<Rgba<u8>>,
                  sublabel: Option<String>,
                  sublabelcolor: Option<Rgba<u8>>,
                  superlabel: Option<String>,
                  superlabelcolor: Option<Rgba<u8>>,
                  defaults: &Defaults) -> Result<(), Error> {
        if color.is_some() {
            self.color = color;
        }
        if file.is_some() {
            self.file = file;
        }
        if label.is_some() || labelcolor.is_some() {
            self.label.map(|mut l| l.update_values(label, labelcolor));
        }
        if sublabel.is_some() || sublabelcolor.is_some() {
            self.label.map(|mut l| l.update_values(sublabel, sublabelcolor));
        }
        if superlabel.is_some() || superlabelcolor.is_some() {
            self.label.map(|mut l| l.update_values(superlabel, superlabelcolor));
        }
        self.draw_face(defaults)
    }

    /// Draws the face from the other values
    fn draw_face(&mut self, defaults: &Defaults) -> Result<(), Error> {
        // Start by creating the face (as rgba image
        // because we want to write rgba data on it).
        let (width, height) = self.device_type.button_image_size();
        let mut face = image::RgbaImage::new(width, height);

        // Get the background color
        let back_color = self.color.unwrap_or(defaults.background_color);

        // Draw on the background color on the face
        imageproc::drawing::draw_filled_rect_mut(
            &mut face,
            imageproc::rect::Rect::at(0, 0).of_size(width, height),
            back_color,
        );

        // Draw the image!
        if let Some(path) = &self.file {
            let top_image = image::io::Reader::open(path)
                .map_err(Error::ImageOpeningError)?
                .decode()
                .map_err(Error::ImageEncodingError)?;
            let top_image = image::imageops::resize(
                &top_image,
                width,
                height,
                image::imageops::FilterType::Lanczos3,
            );
            image::imageops::overlay(&mut face, &top_image, 0, 0);
        }

        // Convert to rgb image
        self.face = image::DynamicImage::ImageRgba8(face).to_rgb8();

        // Draw the text on it
        if let Some(label) = &self.label {
            label.draw(&mut self.face, TextPosition::Center, &defaults.label_color);
        }
        if let Some(sublabel) = &self.sublabel {
            sublabel.draw(&mut self.face, TextPosition::Sub, &defaults.sublabel_color);
        }
        if let Some(superlabel) = &self.superlabel {
            superlabel.draw(
                &mut self.face,
                TextPosition::Super,
                &defaults.superlabel_color,
            );
        }
        Ok(())
    }
}

// Helper functions

/// Find the text scale, so that the given text fits into
/// the given image with.
fn find_text_scale(
    text: &str,
    font: &rusttype::Font,
    image_width: u32,
    default_scale: f32,
) -> (rusttype::Scale, i32, i32) {
    let max_width = image_width as f32 * 0.9;

    let scale = rusttype::Scale::uniform(default_scale);

    let (w, h) = imageproc::drawing::text_size(scale, font, text);
    if w as f32 <= max_width {
        return (scale, w, h);
    }
    let scale = rusttype::Scale::uniform(default_scale * max_width / (w as f32));
    let (w, h) = imageproc::drawing::text_size(scale, font, text);
    (scale, w, h)
}

/// Possible positions of text.
enum TextPosition {
    Center,
    Sub,
    Super,
}

impl ColoredText {
    pub fn from_config(config: &LabelConfig) -> Result<ColoredText, Error> {
        match config {
            LabelConfig::JustText(text) => Ok(ColoredText {
                color: None,
                text: text.clone(),
            }),
            LabelConfig::WithColor(config) => Ok(ColoredText {
                color: match &config.color {
                    None => None,
                    Some(c) => Some(c.to_image_rgba_color().map_err(Error::ConfigError)?),
                },
                text: config.text.clone(),
            }),
        }
    }

    pub fn update_values(&mut self, label: Option<String>, color: Option<Rgba<u8>>) {
        if let Some(label_text) = label {
            self.text = label_text;
        }
        if let Some(label_color) = color {
            self.color = Some(label_color);
        }
    }

    /// Draw the positioned text on the button face.
    fn draw(
        &self,
        image: &mut image::RgbImage,
        position: TextPosition,
        default_color: &image::Rgba<u8>,
    ) {
        // Font data
        let font_data: &[u8] = include_bytes!("../../assets/DejaVuSans.ttf");
        let font = rusttype::Font::try_from_vec(Vec::from(font_data)).unwrap();

        // Find the color, defaulting to the default color
        let color = self.color.as_ref().unwrap_or(default_color);

        let text = &self.text;

        let (scale, w, h) = find_text_scale(
            text.as_str(),
            &font,
            image.width(),
            image.height() as f32
                / match position {
                    TextPosition::Center => 1.1,
                    _ => 4.0,
                },
        );

        let baseline = match position {
            TextPosition::Center => image.height() as f32 / 2.0,
            TextPosition::Sub => image.height() as f32 * 4.0 / 5.0,
            TextPosition::Super => image.height() as f32 / 5.0,
        } as i32;

        imageproc::drawing::draw_text_mut(
            image,
            color.to_rgb(),
            (image.width() as i32 - w) / 2,
            baseline - h / 2,
            scale,
            &font,
            text.as_str(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LabelConfigWithColor;
    use imageproc::assert_pixels_eq;
    use streamdeck_hid_rs::StreamDeckType;

    // Helper function, count pixels with specific color
    fn count_color_occurrences(image: &image::RgbImage, color: &image::Rgb<u8>) -> usize {
        let mut res = 0;
        for x in 0..image.width() {
            for y in 0..image.height() {
                if image.get_pixel(x as u32, y as u32) == color {
                    res += 1;
                }
            }
        }
        res
    }

    #[test]
    fn correct_face_dimensions() {
        // Setup

        // Act
        let face = ButtonFace::from_config(
            &streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: None,
                file: None,
                label: None,
                sublabel: None,
                superlabel: None,
            },
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        assert_eq!(
            face.face.width(),
            StreamDeckType::Orig.button_image_size().0
        );
        assert_eq!(
            face.face.height(),
            StreamDeckType::Orig.button_image_size().1
        );
    }

    #[test]
    fn filled_with_background_color() {
        // Setup

        // Act
        let face = ButtonFace::from_config(
            &streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(String::from("#FF0000"))),
                file: None,
                label: None,
                sublabel: None,
                superlabel: None,
            },
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        let red_image = image::RgbImage::from_pixel(
            face.face.width(),
            face.face.height(),
            image::Rgb([255, 0, 0]),
        );
        assert_pixels_eq!(face.face, red_image);
    }

    #[test]
    fn filled_with_background_image() {
        // Setup
        let back_image_bytes = include_bytes!("./test_image_st_orig.png");
        let back_image = image::load_from_memory(back_image_bytes).unwrap();

        // Act
        let mut face = ButtonFace::from_config(
            &streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(String::from("#FF0000"))),
                file: Some(String::from("./src/state/test_image_st_orig.png")),
                label: None,
                sublabel: None,
                superlabel: None,
            },
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        let mut red_image = image::RgbImage::from_pixel(
            face.face.width(),
            face.face.height(),
            image::Rgb([255, 0, 0]),
        );
        let (width, height) = (face.face.width(), face.face.height());
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, 0, 0, width / 2, height,).to_image(),
            image::imageops::crop(&mut back_image.to_rgb8(), 0, 0, width / 2, height,).to_image()
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, 0, width / 2, height,).to_image(),
            image::imageops::crop(&mut red_image, width / 2, 0, width / 2, height,).to_image()
        );
    }

    #[test]
    fn test_sub_label_colors_appear() {
        // Setup
        let back_image_bytes = include_bytes!("./test_image_st_orig.png");
        let back_image = image::load_from_memory(back_image_bytes).unwrap();

        // Act
        let mut face = ButtonFace::from_config(
            &streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(String::from("#FF0000"))),
                file: Some(String::from("./src/state/test_image_st_orig.png")),
                label: None,
                sublabel: Some(config::LabelConfig::WithColor(LabelConfigWithColor {
                    color: Some(config::ColorConfig::HEXString(String::from("#FFFF00"))),
                    text: String::from("AAAA"),
                })),
                superlabel: None,
            },
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        // Top half of image is original!
        let mut red_image = image::RgbImage::from_pixel(
            face.face.width(),
            face.face.height(),
            image::Rgb([255, 0, 0]),
        );
        let (width, height) = (face.face.width(), face.face.height());
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, 0, 0, width / 2, height / 2,).to_image(),
            image::imageops::crop(&mut back_image.to_rgb8(), 0, 0, width / 2, height / 2,)
                .to_image()
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, 0, width / 2, height / 2,).to_image(),
            image::imageops::crop(&mut red_image, width / 2, 0, width / 2, height / 2,).to_image()
        );
        // Bottom of image should contain yellow pixels
        more_asserts::assert_gt!(
            count_color_occurrences(&face.face, &image::Rgb([255, 255, 0])),
            5
        )
    }

    #[test]
    fn test_super_label_colors_appear() {
        // Setup
        let back_image_bytes = include_bytes!("./test_image_st_orig.png");
        let back_image = image::load_from_memory(back_image_bytes).unwrap();

        // Act
        let mut face = ButtonFace::from_config(
            &streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(String::from("#FF0000"))),
                file: Some(String::from("./src/state/test_image_st_orig.png")),
                label: None,
                sublabel: None,
                superlabel: Some(config::LabelConfig::WithColor(LabelConfigWithColor {
                    color: Some(config::ColorConfig::HEXString(String::from("#FFFF00"))),
                    text: String::from("AAAA"),
                })),
            },
            &Defaults::from_config(&None).unwrap(),
        )
        .unwrap();

        // Test
        // Bottom half of image is original!
        let mut red_image = image::RgbImage::from_pixel(
            face.face.width(),
            face.face.height(),
            image::Rgb([255, 0, 0]),
        );
        let (width, height) = (face.face.width(), face.face.height());
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, 0, height / 2, width / 2, height / 2,).to_image(),
            image::imageops::crop(
                &mut back_image.to_rgb8(),
                0,
                height / 2,
                width / 2,
                height / 2,
            )
            .to_image()
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, height / 2, width / 2, height / 2,)
                .to_image(),
            image::imageops::crop(&mut red_image, width / 2, height / 2, width / 2, height / 2,)
                .to_image()
        );
        // Top of image should contain yellow pixels
        more_asserts::assert_gt!(
            count_color_occurrences(&face.face, &image::Rgb([255, 255, 0])),
            5
        )
    }
}
