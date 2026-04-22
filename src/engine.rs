use winit::{event_loop::EventLoop};

use crate::{world::World};

pub struct Skalora {
    pub world: World,
}

impl Skalora {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(self).unwrap();
    }
}