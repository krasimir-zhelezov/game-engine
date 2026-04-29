mod components;
mod systems;

use skalora_game_engine::{
    components::{
        camera::Camera,
        collider::{Collider, ColliderShape},
        renderable::{Color, Renderable},
        transform::{Position, Scale, Transform},
        velocity::Velocity,
    },
    engine::Skalora,
};

use crate::{
    components::{Ball, Paddle, Player},
    systems::{BallSystem, PaddleSystem},
};

fn main() {
    let mut app = Skalora::new();
    app.world.set_window_title("Ping Pong");

    app.world.components.register_component::<Paddle>();
    app.world.components.register_component::<Ball>();

    app.world.systems.add_system(Box::new(PaddleSystem::new()));
    app.world.systems.add_system(Box::new(BallSystem::new()));

    // Camera
    let camera_id = app.world.spawn_camera();

    // Player 1 Paddle
    let p1 = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        p1,
        Transform {
            position: Position { x: -9.0, y: 0.0 },
            scale: Scale { x: 0.5, y: 3.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        p1,
        Renderable::new_rectangle(Color::from_rgba8(255.0, 255.0, 255.0, 255.0)),
    );
    app.world.components.add_component(
        p1,
        Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 }, // The logic scales this, so a 1x1 box scaled by 0.5x3.0 is correct.
        },
    );
    app.world.components.add_component(
        p1,
        Paddle {
            player: Player::One,
            speed: 0.25,
        },
    );

    // Player 2 Paddle
    let p2 = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        p2,
        Transform {
            position: Position { x: 9.0, y: 0.0 },
            scale: Scale { x: 0.5, y: 3.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        p2,
        Renderable::new_rectangle(Color::from_rgba8(255.0, 255.0, 255.0, 255.0)),
    );
    app.world.components.add_component(
        p2,
        Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
        },
    );
    app.world.components.add_component(
        p2,
        Paddle {
            player: Player::Two,
            speed: 0.25,
        },
    );

    // Ball
    let ball = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        ball,
        Transform {
            position: Position { x: 0.0, y: 0.0 },
            scale: Scale { x: 0.5, y: 0.5 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        ball,
        Renderable::new_circle(Color::from_rgba8(255.0, 50.0, 50.0, 255.0)),
    );
    app.world.components.add_component(
        ball,
        Collider {
            shape: ColliderShape::Circle { radius: 1.0 }, // Radius 1.0 scaled by 0.5 becomes radius 0.5.
        },
    );
    app.world.components.add_component(
        ball,
        Velocity { x: 0.15, y: 0.05 },
    );
    app.world.components.add_component(ball, Ball);

    // Top Wall
    let top_wall = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        top_wall,
        Transform {
            position: Position { x: 0.0, y: 7.0 },
            scale: Scale { x: 22.0, y: 1.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        top_wall,
        Renderable::new_rectangle(Color::from_rgba8(100.0, 100.0, 100.0, 255.0)),
    );
    app.world.components.add_component(
        top_wall,
        Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
        },
    );

    // Bottom Wall
    let bottom_wall = app.world.entity_manager.create_entity();
    app.world.components.add_component(
        bottom_wall,
        Transform {
            position: Position { x: 0.0, y: -7.0 },
            scale: Scale { x: 22.0, y: 1.0 },
            rotation: 0.0,
        },
    );
    app.world.components.add_component(
        bottom_wall,
        Renderable::new_rectangle(Color::from_rgba8(100.0, 100.0, 100.0, 255.0)),
    );
    app.world.components.add_component(
        bottom_wall,
        Collider {
            shape: ColliderShape::Box { width: 1.0, height: 1.0 },
        },
    );

    app.run();
}
