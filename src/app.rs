use std::{sync::Arc, time::{Duration, Instant}};

use winit::window::Window;

use crate::{components::{Color, Renderable, Vec2}, graphics::Graphics};

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
        let mut rectangle = Renderable::new_rectangle(Color { r: 190.0, g: 0.0, b: 201.0, a: 1.0 }, 1.0, 3.5);
        rectangle.transform.position = Vec2 { x: -0.5, y: 0.0 };
        rectangle.transform.scale = Vec2 { x: 0.5, y: 1.5 };

        let mut circle = Renderable::new_circle(Color::BLUE, 0.3);
        circle.transform.scale = Vec2 { x: 0.3, y: 0.3 };
        circle.transform.position = Vec2 { x: 0.5, y: 0.0 };

        if let Some(graphics) = self.graphics.as_mut() {
            graphics.draw_renderables(&mut [&mut circle]);
        }
    }
}

    
