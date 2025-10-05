use std::any::Any;

use crate::world::{World, WorldView};

pub trait System: Any {
    fn update(&mut self, world: &mut WorldView) {}
}

impl dyn System {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}