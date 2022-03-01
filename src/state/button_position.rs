use crate::config;
use streamdeck_hid_rs::StreamDeckType;

/// Position on the Streamdeck (for row or col).
///
/// Allowing defining position as a distance from a border (left, right, top bottom).
#[derive(PartialEq, Debug)]
pub enum PositionFromBorder {
    FromStart(u8),
    FromEnd(u8),
}

impl PositionFromBorder {
    /// Convert to a position form an array index.
    ///
    /// This means that negative indexes are "FromEnd" and positive indexes
    /// (also zero) are "FromStart".
    ///
    /// # Arguments
    ///
    /// index - The "array index" to convert from.
    ///
    /// # Result
    ///
    /// The [PositionFromBorder] object/enum.
    pub fn from_array_index(index: i32) -> PositionFromBorder {
        if index < 0 {
            PositionFromBorder::FromEnd((-index - 1) as u8)
        } else {
            PositionFromBorder::FromStart(index as u8)
        }
    }
}

/// Position of a button
pub struct ButtonPosition {
    pub col: PositionFromBorder,
    pub row: PositionFromBorder,
}

impl ButtonPosition {
    /// Create a button position from the config.
    ///
    /// # Arguments
    ///
    /// config - The config to create the position from.
    ///
    /// # Return
    ///
    /// The button position
    pub fn from_config(config: &config::ButtonPositionConfig) -> ButtonPosition {
        ButtonPosition {
            col: PositionFromBorder::from_array_index(config.col),
            row: PositionFromBorder::from_array_index(config.row),
        }
    }

    pub fn to_button_index(&self, device_type: &StreamDeckType) -> usize {
        let (device_rows, device_cols) = device_type.num_buttons();
        // Convert to row and col without "FromEnd"
        let row = match self.row {
            PositionFromBorder::FromStart(row) => row as i32,
            PositionFromBorder::FromEnd(neg_row) => device_rows as i32 - (neg_row + 1) as i32,
        };
        // Invert col, because the buttons are counted from right to left
        let col = match self.col {
            PositionFromBorder::FromStart(col) => device_cols as i32 - (col + 1) as i32,
            PositionFromBorder::FromEnd(neg_col) => neg_col as i32,
        };
        // Clip row and col
        let row = std::cmp::min(device_rows as i32 - 1, std::cmp::max(0, row));
        let col = std::cmp::min(device_cols as i32 - 1, std::cmp::max(0, col));
        // Return the index
        (col + row * device_cols as i32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_from_border_from_positive_index() {
        // Setup
        let index = 1;
        // Act
        let position = PositionFromBorder::from_array_index(index);
        // Test
        assert_eq!(position, PositionFromBorder::FromStart(1));
    }

    #[test]
    fn position_from_border_from_zero_index() {
        // Setup
        let index = 0;
        // Act
        let position = PositionFromBorder::from_array_index(index);
        // Test
        assert_eq!(position, PositionFromBorder::FromStart(0));
    }

    #[test]
    fn position_from_border_from_negative_index() {
        // Setup
        let index = -1;
        // Act
        let position = PositionFromBorder::from_array_index(index);
        // Test
        assert_eq!(position, PositionFromBorder::FromEnd(0));
    }

    #[test]
    fn top_right_is_index_zero() {
        for device_type in StreamDeckType::ALL {
            // Setup
            let position =
                ButtonPosition::from_config(&config::ButtonPositionConfig { row: 0, col: -1 });
            // Act
            let index = position.to_button_index(&device_type);
            // Test
            assert_eq!(index, 0);
        }
    }

    #[test]
    fn bottom_left_is_last_index() {
        for device_type in StreamDeckType::ALL {
            // Setup
            let position =
                ButtonPosition::from_config(&&config::ButtonPositionConfig { row: -1, col: 0 });
            // Act
            let index = position.to_button_index(&device_type);
            // Test
            assert_eq!(index, device_type.total_num_buttons() - 1);
        }
    }

    #[test]
    fn top_left_is_index_cols() {
        for device_type in StreamDeckType::ALL {
            // Setup
            let position =
                ButtonPosition::from_config(&&config::ButtonPositionConfig { row: 0, col: 0 });
            // Act
            let index = position.to_button_index(&device_type);
            // Test
            assert_eq!(index, device_type.num_buttons().1 as usize - 1);
        }
    }

    #[test]
    fn bottom_right_is_last_index_minus_cols() {
        for device_type in StreamDeckType::ALL {
            // Setup
            let position =
                ButtonPosition::from_config(&&config::ButtonPositionConfig { row: -1, col: -1 });
            // Act
            let index = position.to_button_index(&device_type);
            // Test
            assert_eq!(
                index,
                device_type.total_num_buttons() - device_type.num_buttons().1 as usize
            );
        }
    }
}
