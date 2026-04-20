use std::{sync::Arc};

use crate::{components::component::Component, resources::asset_manager::Texture};

pub enum PrimitiveType {
    Rectangle,
    Circle,
    Line,
}

impl PrimitiveType {
    #[allow(dead_code)]
    fn topology(&self) -> wgpu::PrimitiveTopology {
        match self {
            PrimitiveType::Rectangle => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveType::Circle => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveType::Line => wgpu::PrimitiveTopology::LineList,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_rgba8(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r: r / 255.0, g: g / 255.0, b: b / 255.0, a: a / 255.0}
    }
}

pub enum RenderType {
    Primitive {
        primitive_type: PrimitiveType
    },
    Texture {
        texture: Arc<Texture>
    }
}

pub struct Renderable {
    pub render_type: RenderType,
    pub color: Color,
    pub visible: bool,
}

impl Renderable {
    pub fn new_primitive(primitive_type: PrimitiveType, color: Color) -> Self {
        Self {
            render_type: RenderType::Primitive {
                primitive_type,
            },
            visible: true,
            color,
        }
    }

    pub fn new_rectangle(color: Color) -> Self {
        Self::new_primitive(PrimitiveType::Rectangle, color)
    }

    pub fn new_circle(color: Color) -> Self {
        Self::new_primitive(PrimitiveType::Circle, color)
    }

    pub fn new_line(color: Color) -> Self {
        Self::new_primitive(PrimitiveType::Line, color)
    }

    pub fn new_texture(texture: Arc<Texture>) -> Self {
        Self {
            render_type: RenderType::Texture {
                texture
            },
            color: Color::WHITE, 
            visible: true,
        }
    }
}

impl Component for Renderable {}