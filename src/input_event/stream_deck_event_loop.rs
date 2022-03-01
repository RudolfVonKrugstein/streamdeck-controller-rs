use crate::InputEvent;
use std::sync::Arc;
use std::thread;
use streamdeck_hid_rs::{ButtonState, StreamDeckDevice};

/// Starts a thread getting input events from the device
/// and sending them via the [sender] object.
pub fn run_input_loop_thread(
    device: Arc<StreamDeckDevice<hidapi::HidApi>>,
    sender: std::sync::mpsc::Sender<InputEvent>,
) -> Result<(), streamdeck_hid_rs::Error> {
    let _button_thread = thread::spawn(move || {
        device
            .on_button_events(move |event| match event.state {
                ButtonState::Down => sender
                    .send(InputEvent::ButtonDownEvent(event.button_id))
                    .unwrap(),
                ButtonState::Up => sender
                    .send(InputEvent::ButtonUpEvent(event.button_id))
                    .unwrap(),
            })
            .unwrap();
    });
    Ok(())
}
