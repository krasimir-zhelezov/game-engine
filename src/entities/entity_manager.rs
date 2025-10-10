use crate::entities::entity::Entity;

pub struct EntityManager {
    next_id: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager { 
            next_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity { id: self.next_id };
        self.next_id += 1;
        entity
    }
}