mod foreground_window_event_loop;
mod stream_deck_event_loop;

use crate::foreground_window::WindowInformation;
pub use foreground_window_event_loop::*;
pub use stream_deck_event_loop::*;

#[derive(Debug)]
pub enum InputEvent {
    ButtonDownEvent(u32),
    ButtonUpEvent(u32),
    ForegroundWindow(WindowInformation),
}
