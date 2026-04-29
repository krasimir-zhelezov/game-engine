use skalora_game_engine::{
    components::{transform::Transform, velocity::Velocity},
    resources::collision_events::CollisionEvents,
    systems::{input_system::InputState, system::System},
    world::WorldView,
};
use winit::keyboard::KeyCode;

use crate::components::{Ball, Paddle, Player};

pub struct PaddleSystem;

impl PaddleSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for PaddleSystem {
    fn update(&mut self, world: &mut WorldView) {
        let input = match world.resources.get::<InputState>() {
            Some(i) => i,
            None => return,
        };

        let mut p1_y = 0.0;
        if input.is_key_held(KeyCode::KeyW) {
            p1_y += 1.0;
        }
        if input.is_key_held(KeyCode::KeyS) {
            p1_y -= 1.0;
        }

        let mut p2_y = 0.0;
        if input.is_key_held(KeyCode::ArrowUp) {
            p2_y += 1.0;
        }
        if input.is_key_held(KeyCode::ArrowDown) {
            p2_y -= 1.0;
        }

        let mut active_paddles = Vec::new();
        {
            let paddles = world.components.get_component::<Paddle>();
            for (id, paddle_opt) in paddles.iter().enumerate() {
                if let Some(paddle) = paddle_opt {
                    let dy = if paddle.player == Player::One {
                        p1_y
                    } else {
                        p2_y
                    };
                    active_paddles.push((id, dy, paddle.speed));
                }
            }
        }

        let mut transforms = world.components.get_component_mut::<Transform>();
        for (id, dy, speed) in active_paddles {
            if let Some(Some(transform)) = transforms.get_mut(id) {
                transform.position.y += dy * speed;
                // Clamp position to not go out of bounds (approximate bounds, keeping paddle inside walls)
                if transform.position.y > 5.0 {
                    transform.position.y = 5.0;
                }
                if transform.position.y < -5.0 {
                    transform.position.y = -5.0;
                }
            }
        }
    }
}

pub struct BallSystem;

impl BallSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for BallSystem {
    fn update(&mut self, world: &mut WorldView) {
        let collision_events = {
            let events = world.resources.get::<CollisionEvents>();
            events.map(|e| e.events.iter().map(|ev| (ev.entity_id_a, ev.entity_id_b)).collect::<Vec<_>>()).unwrap_or_default()
        };

        if !collision_events.is_empty() {
            let transforms = world.components.get_component::<Transform>();
            let mut velocities = world.components.get_component_mut::<Velocity>();

            let balls = world.components.get_component::<Ball>();
            let paddles = world.components.get_component::<Paddle>();

            for event in collision_events {
                let id_a = event.0 as usize;
                let id_b = event.1 as usize;

                let is_ball_a = balls.get(id_a).and_then(|o| o.as_ref()).is_some();
                let is_ball_b = balls.get(id_b).and_then(|o| o.as_ref()).is_some();

                let ball_id = if is_ball_a { id_a } else if is_ball_b { id_b } else { continue; };
                let other_id = if is_ball_a { id_b } else { id_a };

                let is_paddle = paddles.get(other_id).and_then(|o| o.as_ref()).is_some();

                let (Some(Some(ball_transform)), Some(Some(other_transform))) = (transforms.get(ball_id), transforms.get(other_id)) else {
                    continue;
                };

                if is_paddle {
                    if let Some(Some(v)) = velocities.get_mut(ball_id) {
                        // Force ball to bounce away from the paddle
                        if other_transform.position.x < 0.0 {
                            v.x = v.x.abs();
                        } else {
                            v.x = -v.x.abs();
                        }

                        // Add some vertical velocity based on where it hit the paddle
                        let hit_factor = (ball_transform.position.y - other_transform.position.y) / (other_transform.scale.y * 0.5);
                        v.y += hit_factor * 0.05;

                        // Cap max speed slightly
                        let mag = (v.x * v.x + v.y * v.y).sqrt();
                        if mag < 0.4 {
                            v.x *= 1.05;
                            v.y *= 1.05;
                        }
                    }
                } else {
                    // Hit a wall
                    if let Some(Some(v)) = velocities.get_mut(ball_id) {
                        // Bounce off top/bottom walls
                        if other_transform.position.y > 0.0 {
                            v.y = -v.y.abs();
                        } else {
                            v.y = v.y.abs();
                        }
                    }
                }
            }
        }

        // Handle bounds / scoring reset
        let mut reset_ball = None;
        {
            let transforms = world.components.get_component::<Transform>();
            let balls = world.components.get_component::<Ball>();
            for (id, ball_opt) in balls.iter().enumerate() {
                if ball_opt.is_some() {
                    if let Some(Some(transform)) = transforms.get(id) {
                        if transform.position.x > 12.0 || transform.position.x < -12.0 {
                            reset_ball = Some((id, transform.position.x > 0.0));
                        }
                    }
                }
            }
        }

        if let Some((id, is_right_side)) = reset_ball {
            if let Some(Some(transform)) = world.components.get_component_mut::<Transform>().get_mut(id) {
                transform.position.x = 0.0;
                transform.position.y = 0.0;
            }
            if let Some(Some(velocity)) = world.components.get_component_mut::<Velocity>().get_mut(id) {
                velocity.x = if is_right_side { -0.15 } else { 0.15 };
                velocity.y = 0.05; 
            }
        }
    }
}
