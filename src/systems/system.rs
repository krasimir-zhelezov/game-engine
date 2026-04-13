use std::any::Any;

use crate::world::{WorldView};

pub trait System: Any {
    fn update(&mut self, _world: &mut WorldView) {}
}

impl dyn System {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}