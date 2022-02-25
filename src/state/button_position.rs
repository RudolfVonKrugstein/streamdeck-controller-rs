use crate::state::button_position::PositionFromBorder::FromEnd;
use streamdeck_hid_rs::StreamDeckType;

/// Position on the Streamdeck (for row or col).
///
/// Allowing defining position as a distance from a border (left, right, top bottom).
#[derive(PartialEq, Debug)]
enum PositionFromBorder {
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
    col: PositionFromBorder,
    row: PositionFromBorder,
}

impl ButtonPosition {
    pub fn to_button_index(&self, device_type: &StreamDeckType) -> usize {
        todo!()
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
}
