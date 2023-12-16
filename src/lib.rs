use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;

use crate::state::State;

mod state;
mod texture;

pub async fn run() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().with_title("Net Sim").build(&event_loop).unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut state = State::new(window).await;

    event_loop.run(move |event, elwt|
        {
            //println!("{:?}", event);

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                event:
                                KeyEvent {
                                    logical_key: Key::Named(NamedKey::Escape),
                                    state: ElementState::Pressed,
                                    ..
                                },
                                ..
                            } => elwt.exit(),
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            // Don't do anything when moving b/c it causes the window to stutter
                            // If in immediate mode this isn't a problem
                            WindowEvent::Moved(_) => {}
                            WindowEvent::RedrawRequested => {
                                state.update();

                                state.window.pre_present_notify();

                                match state.render() {
                                    Ok(_) => {}
                                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                                    Err(e) => { /* eprintln!("{:?}", e) */},
                                }
                            }
                            _ => {
                                state.window.request_redraw();
                            }
                        }
                    }
                }
                // Event::AboutToWait => {
                //     state.window.request_redraw();
                // }
                _ => {}
            }
        }
    )
}