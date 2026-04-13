pub mod app;
pub mod application_handler;
pub mod world;
pub mod systems;
pub mod resources;
pub mod entities;
pub mod components;

use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::app::App;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}