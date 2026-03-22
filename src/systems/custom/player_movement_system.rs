use winit::keyboard::KeyCode;

use crate::{
    components::{
        custom::player_controller::{self, PlayerController},
        tag::Tag,
        transform::Transform,
    },
    systems::{input_system::InputState, system::System},
};

pub struct PlayerMovementSystem;

impl PlayerMovementSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for PlayerMovementSystem {
    fn update(&mut self, world: &mut crate::world::WorldView) {
        let input = world
            .resources
            .get::<InputState>()
            .expect("InputState is missing");

        let mut delta_x = 0.0;
        let mut delta_y = 0.0;

        if input.is_key_held(KeyCode::KeyW) {
            delta_y += 1.0;
        }
        if input.is_key_held(KeyCode::KeyS) {
            delta_y -= 1.0;
        }
        if input.is_key_held(KeyCode::KeyA) {
            delta_x -= 1.0;
        }
        if input.is_key_held(KeyCode::KeyD) {
            delta_x += 1.0;
        }

        if delta_x == 0.0 && delta_y == 0.0 {
            return;
        }

        // * normalize
        // let length = (delta_x * delta_x + delta_y * delta_y).sqrt();

        // if length > 0.0 {
        //     delta_x /= length;
        //     delta_y /= length;
        // }

        let mut active_movements = Vec::new();

        // --- PASS 1: IMMUTABLE SCOPE ---
        // By putting this in a block {}, we force the immutable borrow to drop
        // as soon as the block ends.
        {
            let player_controllers = world.components.get_component::<PlayerController>();
            
            for (id, player_controller_opt) in player_controllers.iter().enumerate() {
                if let Some(player_controller) = player_controller_opt {
                    // Save the ID and the speed
                    active_movements.push((id, player_controller.movement_speed));
                }
            }
        }

        let mut transforms = world.components.get_component_mut::<Transform>();

        for (id, speed) in active_movements {
            // We use the saved `id` to index directly into the transforms array
            if let Some(Some(transform)) = transforms.get_mut(id) {
                transform.position.x += delta_x * speed;
                transform.position.y += delta_y * speed;
            }
        }
    }
}
