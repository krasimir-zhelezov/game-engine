use crate::{components::{transform::Transform, velocity::Velocity}, systems::system::System, world::WorldView};

pub struct VelocitySystem;

impl VelocitySystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for VelocitySystem {
    fn update(&mut self, world: &mut WorldView) {
        
        // let mut transforms = world.components.get_component_mut::<Transform>();
        // let velocities = world.components.get_component::<Velocity>();
    
        // for (id, (transform_opt, velocity_opt)) in transforms.iter_mut().zip(velocities.iter()).enumerate() {
        //     if let (Some(transform), Some(velocity)) = (transform_opt, velocity_opt) {
        //         transform.position.x += velocity.x;
        //         transform.position.y += velocity.y;
        //     }
        // }
    }
}