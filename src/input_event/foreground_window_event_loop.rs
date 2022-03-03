use crate::foreground_window::foreground_window_observer;
use crate::InputEvent;
use std::thread;

/// Starts a thread getting input events about the forground window
/// and sending them via the [sender] object.
pub fn run_foreground_window_event_loop_thread(
    sender: std::sync::mpsc::Sender<InputEvent>,
) -> Result<(), crate::foreground_window::Error> {
    let _wm_thread = thread::spawn(move || {
        foreground_window_observer(move |e| {
            sender
                .send(InputEvent::ForegroundWindow(e.title, e.executable))
                .unwrap();
        })
        .unwrap();
    });
    Ok(())
}
