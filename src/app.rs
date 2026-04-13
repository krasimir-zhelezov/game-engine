use std::{sync::Arc, time::{Duration, Instant}};

use winit::{event_loop::EventLoop, window::Window};

use crate::{world::World};

pub struct App {
    pub world: World,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn run(&mut self) {
        // let event_loop = EventLoop::with_user_event().build().unwrap();
        let event_loop = EventLoop::new().unwrap();

        event_loop.run_app(self).unwrap();
    }
}