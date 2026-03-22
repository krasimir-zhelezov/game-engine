use std::{any::{Any, TypeId}, boxed, cell::{Ref, RefCell, RefMut}, collections::HashMap, hash::Hash};

use crate::{components::{component::Component, tag::Tag}};

pub trait ComponentVec {
    fn remove_entity(&mut self, entity_id: usize);
    
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn remove_entity(&mut self, entity_id: usize) {
        if entity_id < self.len() {
            self[entity_id] = None;
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

pub struct ComponentStore {
    components: HashMap<TypeId, RefCell<Box<dyn ComponentVec>>>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity_id: u32, component: T) {
        let type_id = TypeId::of::<T>();

        let cell = self.components.entry(type_id).or_insert_with(|| RefCell::new(Box::new(Vec::<Option<T>>::new())));

        let mut boxed_vec = cell.borrow_mut();

        let component_vec = boxed_vec.as_any_mut().downcast_mut::<Vec<Option<T>>>().unwrap();

        let index = entity_id as usize;

        if component_vec.len() <= index {
            component_vec.resize_with(index + 1, || None);
        }

        component_vec[index] = Some(component);
    }

    pub fn get_component<T: Component + 'static>(&self) -> Ref<'_, Vec<Option<T>>> {
        let type_id = TypeId::of::<T>();

        let cell = self.components.get(&type_id).expect("Error: No entities with this component found");

        Ref::map(cell.borrow(), |boxed_vec| {
            boxed_vec.as_any().downcast_ref::<Vec<Option<T>>>().unwrap()
        })
    }

    pub fn get_component_mut<T: Component + 'static>(&self) -> RefMut<Vec<Option<T>>> {
        let type_id = TypeId::of::<T>();

        let cell = self.components.get(&type_id).expect("Error: No entities with this component found");

        RefMut::map(cell.borrow_mut(), |boxed_vec| {
            boxed_vec.as_any_mut().downcast_mut::<Vec<Option<T>>>().unwrap()
        })
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        let index = entity_id as usize;
        for cell in self.components.values() {
            cell.borrow_mut().remove_entity(index);
        }
    }
}