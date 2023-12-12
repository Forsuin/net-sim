use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

pub fn run() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, win_target| {
        match event {
            Event::WindowEvent {
                event,
                window_id
            }
            if window_id == window.id() => match event {
                WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                } => win_target.exit(),
                _ => {},
            },
            _ => {},
        }
    })
}