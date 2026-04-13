mod components;
mod systems;

use skalora_game_engine::app::App;
use skalora_game_engine::components::camera::Camera;
use skalora_game_engine::components::renderable::{Color, RenderType, Renderable};
use skalora_game_engine::components::transform::{Position, Scale, Transform};
use skalora_game_engine::world::World;
use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::components::player_controller::PlayerController;
use crate::systems::player_movement_system::PlayerMovementSystem;

fn main() {
    let mut app = App::new();

    app.world
        .components
        .register_component::<PlayerController>();
    app.world
        .systems
        .add_system(Box::new(PlayerMovementSystem::new()));

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
