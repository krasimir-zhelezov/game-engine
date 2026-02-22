use crate::components::component::Component;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
}

impl Component for Tag {}