use serde::{Deserialize};
use crate::config::button_face::ButtonFaceConfig;
use crate::config::event_handler::EventHandlerConfig;


/// Configuration of a button that must have a name
#[derive(Debug, Deserialize, PartialEq)]
struct ButtonConfigWithName {
    name: String,
    up_face: Option<ButtonFaceConfig>,
    down_face: Option<ButtonFaceConfig>,
    up_handler: Option<EventHandlerConfig>,
    down_handler: Option<EventHandlerConfig>
}

/// Configuration of a button that may have no name
#[derive(Debug, Deserialize, PartialEq)]
pub struct ButtonConfigOptionalName {
    name: Option<String>,
    up_face: Option<ButtonFaceConfig>,
    down_face: Option<ButtonFaceConfig>,
    up_handler: Option<EventHandlerConfig>,
    down_handler: Option<EventHandlerConfig>
}

/// Configuration of a button or just the name of a button
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ButtonOrButtonName {
    ButtonName(String),
    Button(ButtonConfigOptionalName)
}

#[cfg(test)]
mod tests {
    use crate::config::color::ColorConfig;
    use crate::config::label::LabelConfig;
    use super::*;

    #[test]
    fn full_button_with_name() {
        // Setup
        let yaml = "\
name: button
up_face:
  color: '#FF0000'
down_face:
  label: Hello
up_handler:
  code: print
down_handler:
  file: handler.py
";

        // Act
        let deserialize: ButtonConfigWithName = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialize.name, "button");
        assert_eq!(deserialize.up_face, Some(ButtonFaceConfig {
            color: Some(ColorConfig::HEXString(String::from("#FF0000"))),
            file: None,
            label: None,
            sublabel: None,
            superlabel: None
        }));
        assert_eq!(deserialize.down_face, Some(ButtonFaceConfig {
            color: None,
            file: None,
            label: Some(LabelConfig::JustText(String::from("Hello"))),
            sublabel: None,
            superlabel: None,
        }));
        assert_eq!(deserialize.up_handler, Some(EventHandlerConfig {
            code: Some(String::from("print")),
            file: None
        }));
        assert_eq!(deserialize.down_handler, Some(EventHandlerConfig {
            code: None,
            file: Some(String::from("handler.py"))
        }));
    }

    #[test]
    fn full_button_optional_name() {
        // Setup
        let yaml = "\
name: button
up_face:
  color: '#FF0000'
down_face:
  label: Hello
up_handler:
  code: print
down_handler:
  file: handler.py
";

        // Act
        let deserialize: ButtonConfigOptionalName = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize.name, Some(String::from("button")));
        assert_eq!(deserialize.up_face, Some(ButtonFaceConfig {
            color: Some(ColorConfig::HEXString(String::from("#FF0000"))),
            file: None,
            label: None,
            sublabel: None,
            superlabel: None
        }));
        assert_eq!(deserialize.down_face, Some(ButtonFaceConfig {
            color: None,
            file: None,
            label: Some(LabelConfig::JustText(String::from("Hello"))),
            sublabel: None,
            superlabel: None,
        }));
        assert_eq!(deserialize.up_handler, Some(EventHandlerConfig {
            code: Some(String::from("print")),
            file: None
        }));
        assert_eq!(deserialize.down_handler, Some(EventHandlerConfig {
            code: None,
            file: Some(String::from("handler.py"))
        }));
    }

    #[test]
    fn force_existences_of_name() {
        // Setup
        let yaml = "\
up_face:
  color: '#FF0000'
down_face:
  label: Hello
up_handler:
  code: print
down_handler:
  file: handler.py
";

        // Act
        let result: Result<ButtonConfigWithName, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_err());
    }

    #[test]
    fn name_is_optional() {
        // Setup
        let yaml = "\
up_face:
  color: '#FF0000'
down_face:
  label: Hello
up_handler:
  code: print
down_handler:
  file: handler.py
";

        // Act
        let result: Result<ButtonConfigOptionalName, serde_yaml::Error> = serde_yaml::from_str(&yaml);

        // Test
        assert!(result.is_ok());
    }

    #[test]
    fn just_button_name() {
        // Setup
        let button_name = "SomeName";
        let yaml = format!("'{}'", button_name);

        // Act
        let deserialize: ButtonOrButtonName = serde_yaml::from_str(&yaml).unwrap();

        // Test
        assert_eq!(deserialize, ButtonOrButtonName::ButtonName(String::from(button_name)));
    }
}
