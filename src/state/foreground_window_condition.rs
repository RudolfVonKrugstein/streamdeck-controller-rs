use crate::config::ForegroundWindowConditionConfig;
use crate::state::error::Error;

/// Condition for actions based on foreground window
pub struct ForegroundWindowCondition {
    pub title: Option<regex::Regex>,
    pub executable: Option<regex::Regex>,
    pub class_name: Option<regex::Regex>,
}

impl ForegroundWindowCondition {
    pub fn from_config(
        config: &ForegroundWindowConditionConfig,
    ) -> Result<ForegroundWindowCondition, Error> {
        let title = match &config.title {
            None => None,
            Some(title) => Some(regex::Regex::new(title.as_str()).map_err(Error::RegexError)?),
        };
        let executable = match &config.executable {
            None => None,
            Some(executable) => {
                Some(regex::Regex::new(executable.as_str()).map_err(Error::RegexError)?)
            }
        };
        let class_name = match &config.class_name {
            None => None,
            Some(class_name) => {
                Some(regex::Regex::new(class_name.as_str()).map_err(Error::RegexError)?)
            }
        };
        Ok(ForegroundWindowCondition {
            title,
            executable,
            class_name,
        })
    }

    /// Test whether the conditions is given by matching the title
    /// and the executable.
    pub fn matches(&self, title: &String, executable: &String, class_name: &String) -> bool {
        let title_matches = if let Some(title_re) = &self.title {
            title_re.is_match(title.as_str())
        } else {
            true
        };
        let exec_matches = if let Some(exec_re) = &self.executable {
            exec_re.is_match(executable.as_str())
        } else {
            true
        };
        let class_matches = if let Some(class_re) = &self.class_name {
            class_re.is_match(class_name.as_str())
        } else {
            true
        };
        title_matches && exec_matches && class_matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_all_matches() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: Some(".*title.*".to_string()),
            executable: Some(".*exec.*".to_string()),
            class_name: Some(".*class.*".to_string()),
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here"),
            &String::from("Some class here"),
        ));
    }

    #[test]
    fn test_with_one_mismatch() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: Some(".*title.*".to_string()),
            executable: Some(".*exec.*".to_string()),
            class_name: Some(".*class.*".to_string()),
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(!object.matches(
            &String::from("No match"),
            &String::from("Some executable here"),
            &String::from("Some class here"),
        ));
        assert!(!object.matches(
            &String::from("Some title here"),
            &String::from("No match"),
            &String::from("Some class here")
        ));
        assert!(!object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here"),
            &String::from("No match")
        ));
    }

    #[test]
    fn test_with_only_title() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: Some(".*title.*".to_string()),
            executable: None,
            class_name: None,
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(!object.matches(
            &String::from("No match"),
            &String::from("Some executable here"),
            &String::from("No match")
        ));
        assert!(object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here"),
            &String::from("No match")
        ));
    }

    #[test]
    fn test_with_only_executable() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: None,
            executable: Some(".*exec.*".to_string()),
            class_name: None,
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(object.matches(
            &String::from("No match"),
            &String::from("Some executable here"),
            &String::from("Some class here")
        ));
        assert!(!object.matches(
            &String::from("Some title here"),
            &String::from("No match"),
            &String::from("Some class here")
        ));
    }

    #[test]
    fn test_with_only_class_name() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: None,
            executable: None,
            class_name: Some(".*class.*".to_string()),
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(object.matches(
            &String::from("No match"),
            &String::from("No match"),
            &String::from("Some class here")
        ));
        assert!(!object.matches(
            &String::from("No match"),
            &String::from("No match"),
            &String::from("No match")
        ));
    }
}
