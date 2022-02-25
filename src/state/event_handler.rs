use super::error::Error;
use crate::config;

/// Event handler, that are executed when an event occurs
///
/// For now its just dummy ...
pub struct EventHandler {
    script: String,
}

impl EventHandler {
    pub fn from_config(config: &config::EventHandlerConfig) -> Result<EventHandler, Error> {
        todo!()
    }
}
