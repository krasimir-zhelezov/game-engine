use crate::{components::{component_store::ComponentStore, tag::Tag}, entities::entity::Entity};

pub struct EntityManager {
    next_id: usize
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager { 
            next_id: 0
        }
    }

    pub fn create_entity(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        return id;
    }
}