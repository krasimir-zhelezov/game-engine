use std::time::{Duration, Instant};

pub struct World {
    pub running: bool,
    pub last_update: Instant,
    pub accumulator: Duration,
    // pub entities: Vec<Entity>, // TODO: Future implementation for entities
}

impl World {
    pub fn new() -> Self {
        Self {
            running: true,
            last_update: Instant::now(),
            accumulator: Duration::ZERO,
            // entities: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        // Update game logic here
    }

    pub fn render(&self) {
        // TODO: render logic
    }

    pub fn handle_keyboard_input(&self) {
        todo!();
    }

    pub fn handdle_mouse_input(&self) {
        todo!();
    }
}