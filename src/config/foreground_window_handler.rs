use crate::config::*;
use serde::Deserialize;

/// Handler for foreground windows
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ForegroundWindowHandlerConfig {
    pub condition: ForegroundWindowConditionConfig,
    pub handler: EventHandlerConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_working_config() {
        // Setup
        let yaml = "\
on_app:
- condition:
    executable: '.*terminal.*'
  handler:
    code: |
      print('terminal')";

        // Act
        let deserialize: serde_yaml::Result<EventHandlerConfig> = serde_yaml::from_str(&yaml);

        // Test
        assert!(deserialize.is_err());
    }
}
