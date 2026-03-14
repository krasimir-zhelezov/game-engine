use std::{any::{Any, TypeId}, boxed, collections::HashMap, hash::Hash};

use crate::{components::{component::Component, tag::Tag}};

pub struct ComponentStore {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity_id: u32, component: T) {
        let type_id = TypeId::of::<T>();

        let boxed_vec = self.components.entry(type_id).or_insert(Box::new(Vec::<Option<T>>::new()));

        let component_vec: &mut Vec<Option<T>> = boxed_vec.downcast_mut::<Vec<Option<T>>>().unwrap();

        let index = entity_id as usize;

        if component_vec.len() <= index {
            component_vec.resize_with(index + 1, || None);
        }

        component_vec[index] = Some(component);
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
}