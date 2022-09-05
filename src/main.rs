use clap::{App, AppSettings, Arg};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn main() {
    // Get information from Cargo.toml
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

    // Argument parsing
    let app = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .author(AUTHORS)
        .arg(
            Arg::with_name("FILE")
                .help("ROM file to run")
                .required(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .help("Display verbose output"),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Display debug info"),
        );

    let matches = app.get_matches();

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("RustBoy");

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
