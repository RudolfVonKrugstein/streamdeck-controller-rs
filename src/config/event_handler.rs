use serde::Deserialize;

/// A label that can be placed on a button.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum EventHandlerConfig {
    AsCode { code: String },
    AsFile { file: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_all_should_not_work() {
        // Setup
        let code_value = "code";
        let file_value = "file";
        let yaml = format!("code: {}\nfile: {}\n", code_value, file_value);

        // Act
        let deserialize: serde_yaml::Result<EventHandlerConfig> = serde_yaml::from_str(&yaml);

        // Test
        assert!(deserialize.is_err());
    }

    #[test]
    fn test_with_only_code() {
        // Setup
        let code_value = "code";
        let yaml = format!("code: {}", code_value);

        // Act
        let deserialize: EventHandlerConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            EventHandlerConfig::AsCode {
                code: String::from(code_value)
            }
        );
    }

    #[test]
    fn test_with_only_file() {
        // Setup
        let file_value = "file";
        let yaml = format!("file: {}", file_value);

        // Act
        let deserialize: EventHandlerConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            EventHandlerConfig::AsFile {
                file: String::from(file_value)
            }
        );
    }
}
