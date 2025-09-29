use std::{sync::Arc, time::{Duration, Instant}};

use wgpu::{wgc::device::queue, wgt::DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, RequestAdapterOptions};
use winit::window::Window;

use crate::graphics::Graphics;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    pub graphics: Option<Graphics>
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            graphics: None
        }
    }

    pub fn update(&mut self) {
        // println!("Updating game logic");
    }

    pub fn render(&mut self) {
        if let Some(graphics) = self.graphics.as_mut() {
            graphics.draw();
        }
    }
}

    
