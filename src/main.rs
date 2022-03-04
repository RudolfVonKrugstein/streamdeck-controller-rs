mod config;
mod foreground_window;
mod input_event;
mod script_engine;
mod state;

use crate::input_event::{
    run_foreground_window_event_loop_thread, run_input_loop_thread, InputEvent,
};
use crate::state::AppState;
use clap::Parser;
use std::fs::File;
use std::sync::Arc;

/// Command line arguments.
///
/// This structure represents the arguments that can be given to the main function.
#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str), short, long, default_value = "./config.yaml")]
    pub config: std::path::PathBuf,
}

fn main() {
    // Parse input arguments
    let args = Cli::parse();

    // Load the config
    let config: config::Config =
        { serde_yaml::from_reader(File::open(&args.config).unwrap()).unwrap() };

    // Detect and open the streamdeck device!
    let hid = hidapi::HidApi::new().unwrap();
    let device = Arc::new(streamdeck_hid_rs::StreamDeckDevice::open_first_device(&hid).unwrap());
    device.reset().unwrap();

    // Initialize the app state
    // Change to the directory of the config
    let config_dir = args.config.as_path().parent().unwrap();
    std::env::set_current_dir(&config_dir).unwrap();
    let mut app_state = AppState::from_config(&device.device_type, &config).unwrap();

    // Create the channels for communication
    let (sender, receiver): (
        std::sync::mpsc::Sender<InputEvent>,
        std::sync::mpsc::Receiver<InputEvent>,
    ) = std::sync::mpsc::channel();

    // Run streamdeck input event thread
    run_input_loop_thread(device.clone(), sender.clone()).unwrap();

    // Run forground window event thread
    run_foreground_window_event_loop_thread(sender.clone()).unwrap();

    // Receive events!
    loop {
        let faces = app_state.set_rendered_and_get_rendering_faces();
        for (button_id, face) in faces {
            device.set_button_image(button_id, &face.face).unwrap();
        }

        let e = receiver.recv().unwrap();
        let handler = match e {
            InputEvent::ButtonDownEvent(button_id) => {
                app_state.on_button_pressed(button_id as usize)
            }
            InputEvent::ButtonUpEvent(button_id) => {
                app_state.on_button_released(button_id as usize)
            }
            InputEvent::ForegroundWindow(info) => {
                // So something
                app_state
                    .on_foreground_window(&info.title, &info.executable, &info.class_name)
                    .unwrap();
                None
            }
        };
        println!("{:?}", handler);

        if let Some(event_handler) = handler {
            let engine = crate::script_engine::PythonEngine::new();

            engine.run_event_handler(&event_handler).unwrap();
        }
    }
}
