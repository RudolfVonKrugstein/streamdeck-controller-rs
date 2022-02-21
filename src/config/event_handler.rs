use serde::{Deserialize};


/// A label that can be placed on a button.
#[derive(Debug, Deserialize, PartialEq)]
pub struct EventHandlerConfig {
    pub code: Option<String>,
    pub file: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_all() {
        // Setup
        let code_value = "code";
        let file_value = "file";
        let yaml = format!("code: {}\nfile: {}\n", code_value, file_value);

        // Act
        let deserialize: EventHandlerConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.code, Some(String::from(code_value)));
        assert_eq!(deserialize.file, Some(String::from(file_value)));
    }
}
