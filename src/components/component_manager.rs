use std::{any::Any, collections::HashMap};

pub struct ComponentManager {
    components: HashMap<u32, Box<dyn Any>>,
}