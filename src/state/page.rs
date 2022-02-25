/// Position on the Streamdeck (for row or col).
///
/// Allowing defining position as a distance from a border (left, right, top bottom).
enum PositionFromBorder {
    FormStart(u8),
    FromEnd(u8),
}

/// Position of a button
struct Position {
    col: PositionFromBorder,
    row: PositionFromBorder,
}

/// Setup of a button with position!
struct PositionedButtonSetup {
    position: Position,
    setup: super::button::ButtonSetupOrName,
}

/// A page, that can be loaded!
struct Page {
    buttons: Vec<PositionedButtonSetup>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
