use skalora_game_engine::{components::{transform::Transform, velocity::Velocity}, resources::collision_events::CollisionEvents, systems::system::System, world::WorldView};

pub struct BounceSystem;

impl BounceSystem {
    pub fn new() -> Self {
        Self
    }
}

impl System for BounceSystem {
    fn update(&mut self, world: &mut WorldView) {
        let collision_events = match world.resources.get::<CollisionEvents>() {
            Some(events) => events,
            None => return,
        };

        if collision_events.events.is_empty() {
            return;
        }

        let transforms = world.components.get_component::<Transform>();
        let mut velocities = world.components.get_component_mut::<Velocity>();

        for event in &collision_events.events {
            let id_a = event.entity_id_a as usize;
            let id_b = event.entity_id_b as usize;

            let (Some(Some(transform_a)), Some(Some(transform_b))) = (transforms.get(id_a), transforms.get(id_b)) else {
                continue;
            };

            let (va_x, va_y) = if let Some(Some(v)) = velocities.get(id_a) { (v.x, v.y) } else { (0.0, 0.0) };
            let (vb_x, vb_y) = if let Some(Some(v)) = velocities.get(id_b) { (v.x, v.y) } else { (0.0, 0.0) };

            let mut nx = transform_a.position.x - transform_b.position.x;
            let mut ny = transform_a.position.y - transform_b.position.y;
            
            let dist_sq = nx * nx + ny * ny;
            if dist_sq == 0.0 {
                nx = 1.0; 
                ny = 0.0;
            } else {
                let dist = dist_sq.sqrt();
                nx /= dist;
                ny /= dist;
            }

            let rvx = va_x - vb_x;
            let rvy = va_y - vb_y;

            let vel_along_normal = rvx * nx + rvy * ny;

            if vel_along_normal > 0.0 {
                continue;
            }

            let restitution = 1.0;

            let j = -(1.0 + restitution) * vel_along_normal;
            let j = j / 2.0;

            let impulse_x = j * nx;
            let impulse_y = j * ny;

            if let Some(Some(v)) = velocities.get_mut(id_a) {
                v.x += impulse_x;
                v.y += impulse_y;
            }
            if let Some(Some(v)) = velocities.get_mut(id_b) {
                v.x -= impulse_x;
                v.y -= impulse_y;
            }
        }
    }
}