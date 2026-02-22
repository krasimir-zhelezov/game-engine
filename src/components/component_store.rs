use std::{any::{Any, TypeId}, boxed, collections::HashMap, hash::Hash};

use crate::{components::{component::Component, tag::Tag}, entities::entity::{self, Entity}};

pub struct ComponentStore {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity_id: usize, component: T) {
        let type_id = TypeId::of::<T>();

        let boxed_vec = self.components.entry(type_id).or_insert(Box::new(Vec::<Option<T>>::new()));

        let component_vec: &mut Vec<Option<T>> = boxed_vec.downcast_mut::<Vec<Option<T>>>().unwrap();

        if component_vec.len() <= entity_id {
            component_vec.resize_with(entity_id + 1, || None);
        }

        component_vec[entity_id] = Some(component);
    }

    pub fn get_component<T: 'static>(&self) -> &Vec<Option<T>> {
        let type_id = TypeId::of::<T>();

        let boxed_vec = self.components.get(&type_id).expect("Error: No entities with this component found!");

        boxed_vec.downcast_ref::<Vec<Option<T>>>().unwrap()
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> &mut Vec<Option<T>> {
        let type_id = TypeId::of::<T>();

        let boxed_vec = self.components.get_mut(&type_id).expect("Error: No entities with this component found!");

        boxed_vec.downcast_mut::<Vec<Option<T>>>().unwrap()
    }

    pub fn get_entities_with_component<T: 'static, U: 'static>(&self) -> Vec<(Entity, &T, &U)> {
        vec![]
        // let type_id_1 = TypeId::of::<T>();
        // let type_id_2 = TypeId::of::<U>();

        // self.entity_components
        //     .iter()
        //     .filter_map(|(entity, comp_types)| {
        //         if comp_types.contains(&type_id_1) && comp_types.contains(&type_id_2) {
        //             Some(*entity)
        //         } else {
        //             None
        //         }
        //     })
        //     .filter_map(|entity| {
        //         if let (Some(comp1), Some(comp2)) = (
        //             self.get_component::<T>(&entity),
        //             self.get_component::<U>(&entity)
        //         ) {
        //             Some((entity, comp1, comp2))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect()
    }
}