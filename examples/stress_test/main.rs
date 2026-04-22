use skalora_game_engine::{
    engine::Skalora,
    components::{
        camera::Camera,
        transform::{Position, Scale, Transform},
    },
};

use crate::systems::stress_test_system::StressTestSystem;

mod systems;

fn main() {
    let mut app = Skalora::new();

    app.world
        .systems
        .add_system(Box::new(StressTestSystem::new(10)));

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

    app.run();
}
