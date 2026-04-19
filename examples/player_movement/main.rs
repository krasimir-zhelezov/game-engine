mod player_controller;
mod player_movement_system;

use skalora_game_engine::{app::App, components::{camera::Camera, renderable::{Color, Renderable}, transform::{Position, Scale, Transform}}};

use crate::{player_controller::PlayerController, player_movement_system::PlayerMovementSystem};

fn main() {
    let mut app = App::new();

    app.world.components.register_component::<PlayerController>();

    app.world.systems.add_system(Box::new(PlayerMovementSystem::new()));

    let camera_id = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        camera_id,
        Transform {
            position: Position { x: 0.0, y: 0.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        camera_id,
        Camera {
            zoom: 10.0,
            aspect_ratio: 4.0 / 3.0,
            near_plane: -100.0,
            far_plane: 100.0,
            fov: 1.0,
        },
    );

    let player_id = app.world.entity_manager.create_entity();

    app.world.components.add_component(
        player_id,
        Transform {
            position: Position { x: 1.0, y: 1.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        },
    );

    app.world.components.add_component(
        player_id,
        PlayerController {
            movement_speed: 0.2,
        },
    );

    app.world.components.add_component::<Renderable>(
        player_id,
        Renderable::new_rectangle(Color::from_rgba8(255.0, 255.0, 0.0, 255.0), 10.0, 10.0),
    );

    app.run();
}