use image::ImageError;
use std::io;

/// Possible errors that can occur during
/// creation and handling of errors
/// within the state.
#[derive(Debug)]
pub enum Error {
    ImageOpeningError(io::Error),
    ImageEncodingError(ImageError),
    ConfigError(crate::config::Error),
    PageNotFound(String),
    LoadScriptFailed(std::io::Error),
    DuplicateNamedButton(String),
}
