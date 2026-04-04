pub struct CollisionEvent {
    pub entity_id_a: u32,
    pub entity_id_b: u32,
}

pub struct CollisionEvents {
    pub events: Vec<CollisionEvent>,
}

impl Default for CollisionEvents {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}