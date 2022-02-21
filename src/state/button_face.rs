use crate::config;
use super::error::Error;

/// Face (picture) to be printed on a button.
///
/// The face is pre-rendered into an image.
pub struct ButtonFace {
    device_type: streamdeck_hid_rs::StreamDeckType,
    pub face: image::RgbImage,
}

impl ButtonFace {
    /// Create a [ButtonFace] from the corresponding
    /// stuff in the configuration ([ButtonFaceConfig]).
    fn from_config(
        device_type: streamdeck_hid_rs::StreamDeckType,
        face_config: &config::ButtonFaceConfig
    ) -> Result<ButtonFace, Error> {
        // Start by creating the face (as rgba image
        // because we want to write rgba data on it).
        let (width, height) = device_type.button_image_size();
        let mut face = image::RgbaImage::new(width, height);

        // Get the background color
        let back_color = match &face_config.color {
            None => image::Rgba([0, 0, 0, 255]),
            Some(c) => c.to_image_rgba_color()
                .map_err(|e| Error::ConfigError(e))?,
        };

        // Draw on the background color on the face
        imageproc::drawing::draw_filled_rect_mut(
            &mut face,
            imageproc::rect::Rect::at(0,0).of_size(width, height),
            back_color
        );

        // Draw the image!
        if let Some(path) = &face_config.file {
            let top_image = image::io::Reader::open(path)
                .map_err(|e| Error::ImageOpeningError(e))?
                .decode()
                .map_err(|e| Error::ImageEncodingError(e))?;
            let top_image = image::imageops::resize(
                &top_image,
                width,
                height,
                image::imageops::FilterType::Lanczos3
            );
            image::imageops::overlay(
                &mut face,
                &top_image,
                0, 0
            );
        }

        // Convert to rgb image
        let face = image::DynamicImage::ImageRgba8(face).into_rgb();

        Ok(
            ButtonFace {
                face: face,
                device_type}
        )
    }
}

#[cfg(test)]
mod tests {
    use image::Pixel;
    use imageproc::drawing::Canvas;
    use streamdeck_hid_rs::StreamDeckType;
    use super::*;

    #[test]
    fn correct_face_dimensions() {
        // Setup

        // Act
        let face = ButtonFace::from_config(
            streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: None,
                file: None,
                label: None,
                sublabel: None,
                superlabel: None
            }
        ).unwrap();

        // Test
        assert_eq!(face.face.width(), StreamDeckType::Orig.button_image_size().0);
        assert_eq!(face.face.height(), StreamDeckType::Orig.button_image_size().1);
    }

    #[test]
    fn filled_with_background_color() {
        // Setup

        // Act
        let face = ButtonFace::from_config(
            streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(
                    String::from("#FF0000")
                )),
                file: None,
                label: None,
                sublabel: None,
                superlabel: None
            }
        ).unwrap();

        // Test
        let (width, height) = streamdeck_hid_rs::StreamDeckType::Orig.button_image_size();
        for x in 0..width {
            for y in 0..height {
                assert_eq!(face.face.get_pixel(x, y).0, [255,0,0])
            }
        }
    }

    #[test]
    fn filled_with_background_image() {
        // Setup
        let back_image_bytes = include_bytes!("./test_image_st_orig.png");
        let back_image = image::load_from_memory(back_image_bytes).unwrap();

        // Act
        let face = ButtonFace::from_config(
            streamdeck_hid_rs::StreamDeckType::Orig,
            &config::ButtonFaceConfig {
                color: Some(config::ColorConfig::HEXString(
                    String::from("#FF0000")
                )),
                file: Some(String::from("./src/state/test_image_st_orig.png")),
                label: None,
                sublabel: None,
                superlabel: None
            }
        ).unwrap();

        // Test
        let (width, height) = streamdeck_hid_rs::StreamDeckType::Orig.button_image_size();
        for x in 0..width {
            for y in 0..height {
                if x >= width / 2 {
                    // On the right size, the image is transparent.
                    // We expect the background.
                    assert_eq!(face.face.get_pixel(x, y).0, [255,0,0])
                } else {
                    assert_eq!(
                        face.face.get_pixel(x, y),
                        &back_image.get_pixel(x, y).to_rgb()
                    )
                }
            }
        }
    }
}
