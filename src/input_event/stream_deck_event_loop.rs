use crate::InputEvent;
use std::thread;
use streamdeck_hid_rs::{ButtonEvent, ButtonState};

/// Starts a thread getting input events from the device
/// and sending them via the [sender] object.
pub fn run_input_loop_thread(
    hid_api: &hidapi::HidApi,
    sender: std::sync::mpsc::Sender<InputEvent>,
) -> Result<(), streamdeck_hid_rs::Error> {
    let device = streamdeck_hid_rs::StreamDeckDevice::open_first_device(hid_api)?;
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
