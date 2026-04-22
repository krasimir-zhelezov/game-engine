mod bounce_system;

use skalora_game_engine::{
    engine::Skalora,
    components::{
        camera::Camera, collider::{Collider, ColliderShape}, renderable::{Color, Renderable}, transform::{Position, Scale, Transform}, velocity::Velocity
    },
};

use crate::bounce_system::BounceSystem;

const BALL_SPEED: f32 = 0.2;

fn main() {
    let mut app = Skalora::new();

    app.world.systems.add_system(Box::new(BounceSystem::new()));

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

    let ball1 = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        ball1,
        Transform {
            position: Position { x: -10.0, y: 0.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        ball1,
        Renderable::new_circle(
            Color::from_rgba8(255.0, 0.0, 0.0, 255.0)
        ),
    );
    app.world.components.add_component(ball1, Collider {
        shape: ColliderShape::Circle { radius: 1.0 },
    });
    app.world.components.add_component(ball1, Velocity {
        x: BALL_SPEED,
        y: 0.0,
    });

    let ball2 = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        ball2,
        Transform {
            position: Position { x: 10.0, y: 0.0 },
            scale: Scale { x: 1.0, y: 1.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        ball2,
        Renderable::new_circle(
            Color::from_rgba8(255.0, 0.0, 255.0, 255.0)
        ),
    );
    app.world.components.add_component(ball2, Collider {
        shape: ColliderShape::Circle { radius: 1.0 },
    });
    app.world.components.add_component(ball2, Velocity {
        x: -BALL_SPEED,
        y: 0.0,
    });

    app.run();
}
