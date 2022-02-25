use super::button_position::ButtonPosition;

/// Setup of a button with position!
struct PositionedButtonSetup {
    position: ButtonPosition,
    setup: super::button::ButtonSetupOrName,
}

/// A page, that can be loaded!
pub struct Page {
    buttons: Vec<PositionedButtonSetup>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
