mod player_controller;
mod player_movement_system;

use skalora_game_engine::{engine::Skalora, components::{camera::Camera, renderable::{Color, Renderable}, transform::{Position, Scale, Transform}}};

use crate::{player_controller::PlayerController, player_movement_system::PlayerMovementSystem};

fn main() {
    let mut engine = Skalora::new();

    engine.world.components.register_component::<PlayerController>();

    engine.world.systems.add_system(Box::new(PlayerMovementSystem::new()));

    let camera_id = engine.world.spawn_camera();

    let player_id = engine.world.entity_manager.create_entity();

    engine.world.components.add_component(
        player_id,
        Transform {
            position: Position { x: 1.0, y: 1.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        },
    );

    engine.world.components.add_component(
        player_id,
        PlayerController {
            movement_speed: 0.2,
        },
    );

    engine.world.components.add_component::<Renderable>(
        player_id,
        Renderable::new_rectangle(Color::from_rgba8(255.0, 255.0, 0.0, 255.0)),
    );

    engine.run();
}