use crate::{
    components::{
        collider::{Collider, ColliderShape},
        transform::{self, Transform},
    }, resources::collision_events::{CollisionEvent, CollisionEvents}, systems::system::System, world::WorldView
};

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for CollisionSystem {
    fn update(&mut self, world: &mut WorldView) {
        let mut collision_events = world.resources.get_mut::<CollisionEvents>().unwrap();

        let transforms = world.components.get_component::<Transform>();
        let colliders = world.components.get_component::<Collider>();

        let mut active_entities = Vec::new();
        for (id, (transform_opt, collider_opt)) in
            transforms.iter().zip(colliders.iter()).enumerate()
        {
            if let (Some(transform), Some(collider)) = (transform_opt, collider_opt) {
                active_entities.push((id, transform, collider));
            }
        }

        for i in 0..active_entities.len() {
            for j in (i + 1)..active_entities.len() {
                let (id_a, transform_a, collider_a) = &active_entities[i];
                let (id_b, transform_b, collider_b) = &active_entities[j];

                if let (
                    ColliderShape::Box {
                        width: width_a,
                        height: height_a,
                    },
                    ColliderShape::Box {
                        width: width_b,
                        height: height_b,
                    },
                ) = (&collider_a.shape, &collider_b.shape)
                {
                    let scaled_width_a = width_a * transform_a.scale.x;
                    let scaled_height_a = height_a * transform_a.scale.y;
                    
                    let scaled_width_b = width_b * transform_b.scale.x;
                    let scaled_height_b = height_b * transform_b.scale.y;

                    let left_a = transform_a.position.x - scaled_width_a / 2.0;
                    let right_a = transform_a.position.x + scaled_width_a / 2.0;
                    let top_a = transform_a.position.y + scaled_height_a / 2.0;
                    let bottom_a = transform_a.position.y - scaled_height_a / 2.0;

                    let left_b = transform_b.position.x - scaled_width_b / 2.0;
                    let right_b = transform_b.position.x + scaled_width_b / 2.0;
                    let top_b = transform_b.position.y + scaled_height_b / 2.0;
                    let bottom_b = transform_b.position.y - scaled_height_b / 2.0;

                    if left_a < right_b && right_a > left_b && top_a > bottom_b && bottom_a < top_b
                    {
                        collision_events.events.push(CollisionEvent {
                            entity_id_a: *id_a as u32,
                            entity_id_b: *id_b as u32,
                        });
                    }
                } else {
                    todo!("Collision detection for non-box colliders not implemented yet");
                }
            }
        }
    }
}
