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

    pub fn get_entities_with_component<T: 'static, U: 'static>(&self) -> Vec<(Entity, &T, &U)> {
        let type_id_1 = TypeId::of::<T>();
        let type_id_2 = TypeId::of::<U>();

        self.entity_components
            .iter()
            .filter_map(|(entity, comp_types)| {
                if comp_types.contains(&type_id_1) && comp_types.contains(&type_id_2) {
                    Some(*entity)
                } else {
                    None
                }
            })
            .filter_map(|entity| {
                if let (Some(comp1), Some(comp2)) = (
                    self.get_component::<T>(&entity),
                    self.get_component::<U>(&entity)
                ) {
                    Some((entity, comp1, comp2))
                } else {
                    None
                }
            })
            .collect()
    }
}