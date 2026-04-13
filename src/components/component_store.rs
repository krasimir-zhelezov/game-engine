use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use crate::components::{component::Component};

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

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ComponentStore {
    components: HashMap<TypeId, RefCell<Box<dyn ComponentVec>>>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components
            .entry(type_id)
            .or_insert_with(|| RefCell::new(Box::new(Vec::<Option<T>>::new())));
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity_id: u32, component: T) {
        let type_id = TypeId::of::<T>();

        let cell = self
            .components
            .entry(type_id)
            .or_insert_with(|| RefCell::new(Box::new(Vec::<Option<T>>::new())));

        let mut boxed_vec = cell.borrow_mut();

        let component_vec = boxed_vec
            .as_any_mut()
            .downcast_mut::<Vec<Option<T>>>()
            .unwrap();

        let index = entity_id as usize;

        if component_vec.len() <= index {
            component_vec.resize_with(index + 1, || None);
        }

        component_vec[index] = Some(component);
    }

    pub fn get_component<T: Component + 'static>(&self) -> Ref<'_, Vec<Option<T>>> {
        let type_id = TypeId::of::<T>();

        let cell = self
            .components
            .get(&type_id)
            .unwrap_or_else(|| panic!("Error: Component is not registered"));

        Ref::map(cell.borrow(), |boxed_vec| {
            boxed_vec.as_any().downcast_ref::<Vec<Option<T>>>().unwrap()
        })
    }

    pub fn get_component_mut<T: Component + 'static>(&self) -> RefMut<'_, Vec<Option<T>>> {
        let type_id = TypeId::of::<T>();

        let cell = self
            .components
            .get(&type_id)
            .expect("Error: Component is not registered");

        RefMut::map(cell.borrow_mut(), |boxed_vec| {
            boxed_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<T>>>()
                .unwrap()
        })
    }

    pub fn remove_entity(&mut self, entity_id: u32) {
        let index = entity_id as usize;
        for cell in self.components.values() {
            cell.borrow_mut().remove_entity(index);
        }
    }

    pub fn remove_component<T: Component + 'static>(&self, entity_id: u32) {
        let type_id = TypeId::of::<T>();

        if let Some(cell) = self.components.get(&type_id) {
            let mut boxed_vec = cell.borrow_mut();

            let component_vec = boxed_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<T>>>()
                .unwrap();

            let index = entity_id as usize;

            if index < component_vec.len() {
                component_vec[index] = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {}

    #[derive(Debug, PartialEq, Clone)]
    struct Health {
        current: i32,
    }
    impl Component for Health {}

    #[test]
    fn test_component_store_remove_entity() {
        let mut store = ComponentStore::new();

        store.register_component::<Position>();
        store.register_component::<Health>();

        let target_entity = 0;
        let bystander_entity = 1;

        store.add_component(target_entity, Position { x: 10.0, y: 20.0 });
        store.add_component(target_entity, Health { current: 100 });

        store.add_component(bystander_entity, Position { x: 50.0, y: 50.0 });
        store.add_component(bystander_entity, Health { current: 50 });

        {
            let positions = store.get_component::<Position>();
            assert!(
                positions[target_entity as usize].is_some(),
                "Target position should exist"
            );
            assert!(
                positions[bystander_entity as usize].is_some(),
                "Bystander position should exist"
            );
        }

        store.remove_entity(target_entity);

        {
            let positions = store.get_component::<Position>();
            let healths = store.get_component::<Health>();

            assert_eq!(
                positions[target_entity as usize], None,
                "Target's Position was not removed"
            );
            assert_eq!(
                healths[target_entity as usize], None,
                "Target's Health was not removed"
            );

            assert_eq!(
                positions[bystander_entity as usize],
                Some(Position { x: 50.0, y: 50.0 }),
                "Bystander's Position was modified unexpectedly!"
            );
            assert_eq!(
                healths[bystander_entity as usize],
                Some(Health { current: 50 }),
                "Bystander's Health was modified unexpectedly!"
            );
        }
    }

    #[test]
    fn test_remove_entity_out_of_bounds() {
        let mut store = ComponentStore::new();
        store.register_component::<Position>();
        store.add_component(0, Position { x: 0.0, y: 0.0 });

        store.remove_entity(999);

        let positions = store.get_component::<Position>();
        assert!(
            positions[0].is_some(),
            "Entity 0 should be unaffected by an out-of-bounds removal"
        );
    }

    #[test]
    fn test_remove_component_from_entity() {
        let mut store = ComponentStore::new();
        store.register_component::<Position>();
        store.add_component(0, Position { x: 0.0, y: 0.0 });
        store.add_component(1, Position { x: 0.0, y: 0.0 });

        store.remove_component::<Position>(0);

        {
            let positions = store.get_component::<Position>();
            assert!(
                positions[0].is_none(),
                "Entity 0 should not have position component"
            );
            assert!(
                positions[1].is_some(),
                "Entity 1 should have position component"
            );
        }
        
        store.remove_component::<Position>(99);
    }
}
