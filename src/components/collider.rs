use crate::components::component::Component;

pub enum ColliderShape {
    Box { width: f32, height: f32 },
    Circle { radius: f32 },
}

pub struct Collider {
    pub shape: ColliderShape,
}

impl Component for Collider {}