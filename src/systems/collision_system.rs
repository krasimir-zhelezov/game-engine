use crate::{
    components::{
        collider::{Collider, ColliderShape},
        transform::{Transform},
    }, resources::collision_events::{CollisionEvent, CollisionEvents}, systems::system::System, world::WorldView
};

/// A system responsible for detecting physical overlaps between entities.
///
/// The `CollisionSystem` iterates through all entities that possess both a
/// [`Transform`] and a [`Collider`] component. It calculates their world-space
/// bounding boxes and checks for intersections. When a valid overlap is detected,
/// a [`CollisionEvent`] is generated and appended to the [`CollisionEvents`] resource.
pub struct CollisionSystem;

impl CollisionSystem {
    /// Creates a new instance of the `CollisionSystem`.
    pub fn new() -> Self {
        Self
    }
}

impl System for CollisionSystem {
    /// Executes the collision detection logic for the current tick/frame.
    ///
    /// # Algorithm
    /// This uses an Axis-Aligned Bounding Box (AABB) intersection test. It calculates 
    /// the left, right, top, and bottom edges of each entity based on its `Transform` 
    /// (position and scale) and `ColliderShape::Box` dimensions. 
    ///
    /// It performs a pairwise comparison between all eligible entities. 
    ///
    /// # Panics
    /// This function will currently panic with a `todo!` if it encounters any collider 
    /// shape other than a `ColliderShape::Box`, as non-box intersections are not yet implemented.
    fn update(&mut self, world: &mut WorldView) {
        // Fetch the collision events resource to populate
        let collision_events = world.resources.get_mut::<CollisionEvents>().unwrap();

        // Fetch the necessary component pools
        let transforms = world.components.get_component::<Transform>();
        let colliders = world.components.get_component::<Collider>();

        // Pre-filter entities that have both a Transform and a Collider
        let mut active_entities = Vec::new();
        for (id, (transform_opt, collider_opt)) in
            transforms.iter().zip(colliders.iter()).enumerate()
        {
            if let (Some(transform), Some(collider)) = (transform_opt, collider_opt) {
                active_entities.push((id, transform, collider));
            }
        }

        // Pairwise comparison of all active entities
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
                    // Calculate dimensions incorporating the entity's current scale
                    let scaled_width_a = width_a * transform_a.scale.x;
                    let scaled_height_a = height_a * transform_a.scale.y;
                    
                    let scaled_width_b = width_b * transform_b.scale.x;
                    let scaled_height_b = height_b * transform_b.scale.y;

                    // Calculate AABB edges for entity A
                    let left_a = transform_a.position.x - scaled_width_a / 2.0;
                    let right_a = transform_a.position.x + scaled_width_a / 2.0;
                    let top_a = transform_a.position.y + scaled_height_a / 2.0;
                    let bottom_a = transform_a.position.y - scaled_height_a / 2.0;

                    // Calculate AABB edges for entity B
                    let left_b = transform_b.position.x - scaled_width_b / 2.0;
                    let right_b = transform_b.position.x + scaled_width_b / 2.0;
                    let top_b = transform_b.position.y + scaled_height_b / 2.0;
                    let bottom_b = transform_b.position.y - scaled_height_b / 2.0;

                    // AABB intersection check
                    if left_a < right_b && right_a > left_b && top_a > bottom_b && bottom_a < top_b
                    {
                        // Overlap detected; record the collision event
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
