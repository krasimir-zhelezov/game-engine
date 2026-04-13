use crate::{components::{renderable::{Color, Renderable}, transform::{Position, Scale, Transform}}, systems::system::System, world::WorldView};
use rand::prelude::*;

pub struct StressTestSystem {
    pub spawn_rate_per_frame: u32,
    pub active: bool
}

impl StressTestSystem {
    pub fn new(spawn_rate_per_frame: u32) -> Self {
        StressTestSystem {
            spawn_rate_per_frame,
            active: true,
        }
    }
}

impl System for StressTestSystem {
    fn update(&mut self, world: &mut WorldView) {
        if !self.active {
            return;
        }
        
        let mut rng = rand::rng();

        for _ in 0..self.spawn_rate_per_frame {
            let id = world.entity_manager.create_entity();
            world.components.add_component(id, Transform {
                position: Position {
                    x: rng.random_range(-10.0..10.00),
                    y: rng.random_range(-10.0..10.00),
                },
                scale: Scale { x: 1.0, y: 1.0 },
                rotation: 0.0,
            });
            world.components.add_component(id, Renderable::new_rectangle(
                Color {
                    r: rng.random_range(0..255) as f32,
                    g: rng.random_range(0..255) as f32,
                    b: rng.random_range(0..255) as f32,
                    a: 255.0,
                },
                5.0,
                5.0
            ));
        }
    }
}