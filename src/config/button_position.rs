use serde::Deserialize;

/// Button positions can be given as tuples ar os objects!
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ButtonPositionConfig {
    ButtonPositionTupleConfig(String),
    ButtonPositionObjectConfig(ButtonPositionObject),
}

/// Position of a button on a page.
///
/// [row] and [col] can be negative, setting the position counting from right or below.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ButtonPositionObject {
    pub row: i32,
    pub col: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_positions() {
        // Setup
        let yaml = "row: 0\ncol: 1\n";

        // Act
        let deserialize: ButtonPositionConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            ButtonPositionConfig::ButtonPositionObjectConfig(ButtonPositionObject {
                row: 0,
                col: 1
            })
        );
    }

    #[test]
    fn negative_positions() {
        // Setup
        let yaml = "row: -1\ncol: -2\n";

        // Act
        let deserialize: ButtonPositionConfig = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(
            deserialize,
            ButtonPositionConfig::ButtonPositionObjectConfig(ButtonPositionObject {
                row: -1,
                col: -2
            })
        );
    }

    #[test]
    fn missing_position() {
        // Setup
        let yaml = "row: -1\n";

        // Act
        let result: Result<ButtonPositionConfig, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_err());
    }
}
