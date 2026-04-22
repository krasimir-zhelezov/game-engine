use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{self, Window},
};

use crate::{engine::Skalora};

const FPS: u32 = 60;
const FRAME_DURATION: std::time::Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

impl ApplicationHandler for Skalora {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.world.window.is_none() {
            let mut attributes = Window::default_attributes();

            attributes.title = "Skalora Game Engine".to_string();

            let window = Arc::new(event_loop.create_window(attributes).unwrap());

            self.world.init_graphics(window.clone());
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
            self.world.running = false;
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta = now - self.world.last_update;
                self.world.last_update = now;
                self.world.accumulator += delta;

                while self.world.accumulator >= FRAME_DURATION {
                    self.world.update();
                    self.world.accumulator -= FRAME_DURATION;
                }

                if let Some(window) = &self.world.window {
                    if self.world.running {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.world.handle_keyboard_input(&event);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.world.handle_mouse_button(state, button);
            }
            WindowEvent::Resized(_new_size) => {}
            WindowEvent::CursorMoved { position, .. } => {
                self.world.handle_cursor_moved(position);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.world.handle_mouse_wheel(delta);
            }
            _ => {}
        }
    }
}