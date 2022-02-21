use std::io;
use image::ImageError;

/// Possible errors that can occur during
/// creation and handling of errors
/// within the state.
#[derive(Debug)]
pub enum Error {
    ImageOpeningError(io::Error),
    ImageEncodingError(ImageError),
    ConfigError(crate::config::Error)
}
