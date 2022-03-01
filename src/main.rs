mod config;
mod input_event;
mod state;

use crate::input_event::{run_input_loop_thread, InputEvent};
use crate::state::AppState;
use clap::Parser;
use std::fs::File;

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
    let device = streamdeck_hid_rs::StreamDeckDevice::open_first_device(&hid).unwrap();
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
    run_input_loop_thread(&hid, sender.clone()).unwrap();

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
        };
        println!("{:?}, {:?}", e, handler);
    }
}
