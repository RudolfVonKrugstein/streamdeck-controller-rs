use crate::config::ForegroundWindowConditionConfig;
use crate::state::error::Error;

/// Condition for actions based on foreground window
pub struct ForegroundWindowCondition {
    pub title: Option<regex::Regex>,
    pub executable: Option<regex::Regex>,
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
        Ok(ForegroundWindowCondition { title, executable })
    }

    /// Test whether the conditions is given by matching the title
    /// and the executable.
    pub fn matches(&self, title: &String, executable: &String) -> bool {
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
        title_matches && exec_matches
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
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here")
        ));
    }

    #[test]
    fn test_with_one_mismatch() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: Some(".*title.*".to_string()),
            executable: Some(".*exec.*".to_string()),
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(!object.matches(
            &String::from("No match"),
            &String::from("Some executable here")
        ));
        assert!(!object.matches(&String::from("Some title here"), &String::from("No match")));
    }

    #[test]
    fn test_with_only_title() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: Some(".*title.*".to_string()),
            executable: None,
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(!object.matches(
            &String::from("No match"),
            &String::from("Some executable here")
        ));
        assert!(object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here")
        ));
    }

    #[test]
    fn test_with_only_executable() {
        // Setup
        let config = crate::config::ForegroundWindowConditionConfig {
            title: None,
            executable: Some(".*exec.*".to_string()),
        };

        // Act
        let object = ForegroundWindowCondition::from_config(&config).unwrap();

        // Test
        assert!(object.matches(
            &String::from("No match"),
            &String::from("Some executable here")
        ));
        assert!(!object.matches(
            &String::from("Some title here"),
            &String::from("Some executable here")
        ));
    }
}
