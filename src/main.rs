use std::{sync::Arc, time::{Duration, Instant}};

use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{self, EventLoop}, window::{self, Window, WindowAttributes}};

const FPS: u32 = 60;
const FRAME_DURATION: std::time::Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

struct App {
    window: Option<Arc<Window>>,
    running: bool,
    last_update: Instant,
    accumulator: Duration,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }

    fn update(&mut self) {
        println!("Updating game logic");
    }

    fn render(&self) {
        println!("Rendering frame");
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut attributes = Window::default_attributes();

        attributes.title = "Game Engine 2D".to_string();

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        self.window = Some(window);
    }  

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            self.running = false;
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta = now - self.last_update;
                self.last_update = now;
                self.accumulator += delta;

                while self.accumulator >= FRAME_DURATION {
                    self.update();
                    self.accumulator -= FRAME_DURATION;
                }

                self.render();

                if let Some(window) = &self.window {
                    if self.running {
                        window.request_redraw();
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}