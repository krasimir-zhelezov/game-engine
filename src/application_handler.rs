use std::{sync::Arc, time::{Duration, Instant}};

use winit::{application::ApplicationHandler, event::WindowEvent, window::{self, Fullscreen, Window}};

use crate::{app::App, graphics::init_graphics};

const FPS: u32 = 60;
const FRAME_DURATION: std::time::Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut attributes = Window::default_attributes();

        attributes.title = "Game Engine 2D".to_string();

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        // let primary_monitor = window.available_monitors().next().unwrap();

        // window.set_fullscreen(Some(Fullscreen::Borderless(Some(primary_monitor))));

        self.graphics = pollster::block_on(init_graphics(window.clone()));
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
                    self.world.render();
                    self.world.accumulator -= FRAME_DURATION;
                }

                if let Some(window) = &self.window {
                    if self.world.running {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                println!("Key event: {:?}", event);
            }
            WindowEvent::MouseInput { device_id, state, button } => {
                println!("Mouse button event: {:?}", button);
            }
            WindowEvent::Resized(new_size) => {
                if let Some(graphics) = self.graphics.as_mut() {
                    graphics.resize(new_size);
                }
            }
            _ => {}
        }
    }
}