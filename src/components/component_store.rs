use std::{any::{Any, TypeId}, collections::HashMap};

use crate::entities::entity::{self, Entity};

pub type ComponentMap = HashMap<Entity, Box<dyn Any>>; // Entity to Component

pub struct ComponentStore {
    components: HashMap<TypeId, ComponentMap>,
    entity_components: HashMap<Entity, Vec<TypeId>>, // Entity to Component Vectors
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            entity_components: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        
        self.components
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));   
        
        self.entity_components
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(type_id);
    }

    pub fn get_component<T: 'static>(&self, entity: &Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)?
            .get(&entity)?
            .downcast_ref::<T>()
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: &Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)?
            .get_mut(&entity)?
            .downcast_mut::<T>()
    }
}