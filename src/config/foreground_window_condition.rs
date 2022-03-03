use serde::Deserialize;

/// Condition for actions based on foreground window
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ForegroundWindowConditionConfig {
    pub title: Option<String>,
    pub executable: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_all() {
        // Setup
        let title_value = ".*title.*";
        let executable_value = ".*exec.*";
        let yaml = format!("title: {}\nexecutable: {}\n", title_value, executable_value);

        // Act
        let deserialize: ForegroundWindowConditionConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.title, Some(title_value.to_string()));
        assert_eq!(deserialize.executable, Some(executable_value.to_string()));
    }

    #[test]
    fn test_with_only_title() {
        // Setup
        let title_value = ".*title.*";
        let yaml = format!("title: {}\n", title_value);

        // Act
        let deserialize: ForegroundWindowConditionConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.title, Some(title_value.to_string()));
        assert_eq!(deserialize.executable, None);
    }

    #[test]
    fn test_with_only_executable() {
        // Setup
        let exec_value = ".*executable.*";
        let yaml = format!("executable: {}\n", exec_value);

        // Act
        let deserialize: ForegroundWindowConditionConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.title, None);
        assert_eq!(deserialize.executable, Some(exec_value.to_string()));
    }
}
