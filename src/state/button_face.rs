use super::error::Error;
use crate::config;
use image::Pixel;

/// Face (picture) to be printed on a button.
///
/// The face is pre-rendered into an image.
pub struct ButtonFace {
    _device_type: streamdeck_hid_rs::StreamDeckType,
    pub face: image::RgbImage,
}

impl ButtonFace {
    /// Create a [ButtonFace] from the corresponding
    /// stuff in the configuration ([ButtonFaceConfig]).
    pub fn from_config(
        device_type: &streamdeck_hid_rs::StreamDeckType,
        face_config: &config::ButtonFaceConfig,
    ) -> Result<ButtonFace, Error> {
        // Start by creating the face (as rgba image
        // because we want to write rgba data on it).
        let (width, height) = device_type.button_image_size();
        let mut face = image::RgbaImage::new(width, height);

        // Get the background color
        let back_color = match &face_config.color {
            None => image::Rgba([0, 0, 0, 255]),
            Some(c) => c.to_image_rgba_color().map_err(Error::ConfigError)?,
        };

        // Draw on the background color on the face
        imageproc::drawing::draw_filled_rect_mut(
            &mut face,
            imageproc::rect::Rect::at(0, 0).of_size(width, height),
            back_color,
        );

        // Draw the image!
        if let Some(path) = &face_config.file {
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
        let mut face = image::DynamicImage::ImageRgba8(face).to_rgb8();

        // Draw the text on it
        if let Some(label_text) = &face_config.label {
            draw_positioned_colored_text(&mut face, label_text, TextPosition::Center);
        }
        if let Some(label_text) = &face_config.sublabel {
            draw_positioned_colored_text(&mut face, label_text, TextPosition::Sub);
        }
        if let Some(label_text) = &face_config.superlabel {
            draw_positioned_colored_text(&mut face, label_text, TextPosition::Super);
        }

        let device_type = device_type.clone();
        Ok(ButtonFace {
            face,
            _device_type: device_type,
        })
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

/// Draw the positioned text on the button face.
fn draw_positioned_colored_text(
    image: &mut image::RgbImage,
    label: &config::LabelConfig,
    position: TextPosition,
) {
    // Font data
    let font_data: &[u8] = include_bytes!("../../assets/DejaVuSans.ttf");
    let font = rusttype::Font::try_from_vec(Vec::from(font_data)).unwrap();

    // Find the color, defaulting to white
    let color = match label {
        config::LabelConfig::JustText(_) => image::Rgba([255, 255, 255, 255]),
        config::LabelConfig::WithColor(c) => match &c.color {
            None => image::Rgba([255, 255, 255, 255]),
            Some(c) => c
                .to_image_rgba_color()
                .unwrap_or(image::Rgba([255, 255, 255, 255])),
        },
    };

    let text = match label {
        config::LabelConfig::JustText(s) => s,
        config::LabelConfig::WithColor(c) => &c.text,
    };

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LabelConfigWithColor;
    use imageproc::assert_pixels_eq;
    use imageproc::drawing::Canvas;
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
            image::imageops::crop(&mut face.face, 0, 0, width / 2, height,),
            image::imageops::crop(&mut back_image.to_rgb8(), 0, 0, width / 2, height,)
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, 0, width / 2, height,),
            image::imageops::crop(&mut red_image, width / 2, 0, width / 2, height,)
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
            image::imageops::crop(&mut face.face, 0, 0, width / 2, height / 2,),
            image::imageops::crop(&mut back_image.to_rgb8(), 0, 0, width / 2, height / 2,)
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, 0, width / 2, height / 2,),
            image::imageops::crop(&mut red_image, width / 2, 0, width / 2, height / 2,)
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
            image::imageops::crop(&mut face.face, 0, height / 2, width / 2, height / 2,),
            image::imageops::crop(
                &mut back_image.to_rgb8(),
                0,
                height / 2,
                width / 2,
                height / 2,
            )
        );
        assert_pixels_eq!(
            image::imageops::crop(&mut face.face, width / 2, height / 2, width / 2, height / 2,),
            image::imageops::crop(&mut red_image, width / 2, height / 2, width / 2, height / 2,)
        );
        // Top of image should contain yellow pixels
        more_asserts::assert_gt!(
            count_color_occurrences(&face.face, &image::Rgb([255, 255, 0])),
            5
        )
    }
}
