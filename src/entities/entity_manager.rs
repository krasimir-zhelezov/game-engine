use crate::{components::{component_store::ComponentStore, tag::Tag}, entities::entity::Entity};

pub struct EntityManager {
    next_id: u32,
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager { 
            next_id: 0,
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity { id: self.next_id };
        self.entities.push(entity);
        self.next_id += 1;
        entity
    }

    pub fn get_entities_by_tag(&self, tag: &str, component_store: &ComponentStore) -> Vec<Entity> {
        let mut tagged_entities = Vec::new();
        for entity in &self.entities {
            if let Some(tag) = component_store.get_component::<Tag>(entity) {
                if tag.name == "Player" {
                    tagged_entities.push(*entity);
                }
            }
        }
        
        tagged_entities
    }
    
    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}