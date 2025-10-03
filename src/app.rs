use std::{sync::Arc, time::{Duration, Instant}};

use winit::window::Window;

use crate::{components::{Color, Renderable}, graphics::Graphics};

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
        let mut rectangle = Renderable::new_rectangle(Color { r: 190.0, g: 0.0, b: 201.0, a: 0.3 }, 1.0, 3.5);
        rectangle.transform.position = [1.0, 0.5];
        rectangle.transform.scale = [0.5, 0.5];

        // let mut circle = Renderable::new_circle(Color::BLUE, 0.3);
        // circle.transform.scale = [0.3, 0.3];

        if let Some(graphics) = self.graphics.as_mut() {
            graphics.draw_renderables(&mut [&mut rectangle]);
        }
    }
}

    
