use super::error::Error;
use crate::config;
use crate::config::EventHandlerConfig;
use std::fs;

/// Event handler, that are executed when an event occurs
///
/// For now its just dummy ...
pub struct EventHandler {
    pub script: String,
}

impl EventHandler {
    pub fn from_config(config: &config::EventHandlerConfig) -> Result<EventHandler, Error> {
        Ok(match config {
            EventHandlerConfig::AsCode { code } => EventHandler {
                script: code.clone(),
            },
            EventHandlerConfig::AsFile { file } => EventHandler {
                script: fs::read_to_string(&file).map_err(Error::LoadScriptFailed)?,
            },
        })
    }
}
