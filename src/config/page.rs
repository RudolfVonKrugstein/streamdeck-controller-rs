use crate::config::button::ButtonOrButtonName;
use crate::config::button_position::ButtonPositionConfig;
use crate::config::ForegroundWindowConditionConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PageConfig {
    pub name: String,
    pub on_app: Option<PageLoadConditions>,
    pub buttons: Vec<PageButtonConfig>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PageButtonConfig {
    pub position: ButtonPositionConfig,
    pub button: ButtonOrButtonName,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PageLoadConditions {
    pub conditions: Vec<ForegroundWindowConditionConfig>,
    pub remove: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::button::ButtonOrButtonName;

    #[test]
    fn page_button_config() {
        // Setup
        let yaml = "\
position:
  row: 0
  col: 1
button: button1
";

        // Act
        let deserialize: PageButtonConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            PageButtonConfig {
                position: ButtonPositionConfig { row: 0, col: 1 },
                button: ButtonOrButtonName::ButtonName(String::from("button1"))
            }
        );
    }

    #[test]
    fn page_button_missing_button() {
        // Setup
        let yaml = "\
position:
  row: 0
  col: 1
";

        // Act
        let result: Result<PageButtonConfig, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn page_config() {
        // Setup
        let yaml = "\
name: page1
buttons:
- position:
    row: 0
    col: 1
  button: button1
";

        // Act
        let deserialize: PageConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            PageConfig {
                name: String::from("page1"),
                on_app: None,
                buttons: Vec::from([PageButtonConfig {
                    position: ButtonPositionConfig { row: 0, col: 1 },
                    button: ButtonOrButtonName::ButtonName(String::from("button1"))
                }])
            }
        );
    }

    #[test]
    fn page_config_with_on_app() {
        // Setup
        let yaml = "\
name: page1
on_app:
  conditions:
  - executable: '.*exec.*'
    title: '.*title.*'
buttons:
- position:
    row: 0
    col: 1
  button: button1
";

        // Act
        let deserialize: PageConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            PageConfig {
                name: String::from("page1"),
                on_app: Some(PageLoadConditions {
                    conditions: vec![ForegroundWindowConditionConfig {
                        title: Some(".*title.*".to_string()),
                        executable: Some(".*exec.*".to_string()),
                        class_name: None,
                    }],
                    remove: None
                }),
                buttons: Vec::from([PageButtonConfig {
                    position: ButtonPositionConfig { row: 0, col: 1 },
                    button: ButtonOrButtonName::ButtonName(String::from("button1"))
                }])
            }
        );
    }
}
