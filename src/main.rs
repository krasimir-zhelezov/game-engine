use std::sync::Arc;

use winit::{application::ApplicationHandler, error::EventLoopError, event::WindowEvent, event_loop::{self, EventLoop}, window::{self, Window, WindowAttributes}};

struct App {
    window: Option<Arc<Window>>
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut attributes = Window::default_attributes();

        attributes.title = "Game Engine 2D".to_string();

        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        // window.request_redraw();
        // window.focus_window();
        self.window = Some(window);
    }  

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                println!("Redraw")
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