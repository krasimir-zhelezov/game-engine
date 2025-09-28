mod  app;
mod application_handler;

use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::app::App;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}