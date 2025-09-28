use std::{sync::Arc, time::{Duration, Instant}};

use winit::window::Window;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }

    pub fn update(&mut self) {
        println!("Updating game logic");
    }

    pub fn render(&self) {
        println!("Rendering frame");
    }
}