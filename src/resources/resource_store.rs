use std::{any::{Any, TypeId}, collections::HashMap};

pub struct ResourceStore {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl ResourceStore {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources.get_mut(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_mut::<T>())
    }
}