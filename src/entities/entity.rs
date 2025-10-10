use std::any::Any;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity {
    pub id: u32
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}