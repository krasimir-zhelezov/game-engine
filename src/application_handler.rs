use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{self, Fullscreen, Window},
};

use crate::{app::App, world::World};

const FPS: u32 = 60;
const FRAME_DURATION: std::time::Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut attributes = Window::default_attributes();

        attributes.title = "Skalora Game Engine".to_string();

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        // let primary_monitor = window.available_monitors().next().unwrap();

        // window.set_fullscreen(Some(Fullscreen::Borderless(Some(primary_monitor))));

        self.world = Some(World::new(window.clone()));
        self.window = Some(window);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            if let Some(world) = &mut self.world {
                world.running = false;
            }
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                if let Some(world) = &mut self.world {
                    let now = Instant::now();
                    let delta = now - world.last_update;
                    world.last_update = now;
                    world.accumulator += delta;

                    while world.accumulator >= FRAME_DURATION {
                        world.update();
                        world.accumulator -= FRAME_DURATION;
                    }

                    if let Some(window) = &self.window {
                        if world.running {
                            window.request_redraw();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let Some(world) = &mut self.world {
                    world.handle_keyboard_input(&event);
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => {
                println!("Mouse button event: {:?}", button);
            }
            WindowEvent::Resized(new_size) => {}
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(world) = &mut self.world {
                    world.handle_mouse_button(state, button);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(world) = &mut self.world {
                    world.handle_cursor_moved(position);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(world) = &mut self.world {
                    world.handle_mouse_wheel(delta);
                }
            }
            _ => {}
        }
    }
}
