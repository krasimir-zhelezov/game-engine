use crate::{systems::system::System, world::WorldView};

pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn update(&mut self, world: &mut WorldView) {
        for system in &mut self.systems {
            system.update(world);
        }
    }

    pub fn get_system<T: System>(&self) -> Option<&T> {
        for system in &self.systems {
            if let Some(s) = system.as_any().downcast_ref::<T>() {
                return Some(s);
            }
        }
        None
    }

    pub fn get_system_mut<T: System>(&mut self) -> Option<&mut T> {
        for system in &mut self.systems {
            if let Some(s) = system.as_any_mut().downcast_mut::<T>() {
                return Some(s);
            }
        }
        None
    }
}