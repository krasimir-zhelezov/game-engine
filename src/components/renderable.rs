use std::path::Path;

use wgpu::Buffer;

use crate::{components::component::Component, resources::asset_manager::Texture};

pub enum PrimitiveType {
    Rectangle,
    Circle,
    Line,
}

impl PrimitiveType {
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
}

pub enum RenderType {
    Primitive {
        primitive_type: PrimitiveType,
        parameters: [f32; 4],
    },
    Texture {
        image_data: Vec<u8>,
        width: u32,
        height: u32,
    }
}

pub struct Renderable {
    pub render_type: RenderType,
    pub color: Color,
    pub visible: bool,
}

impl Renderable {
    pub fn new_primitive(primitive_type: PrimitiveType, color: Color, parameters: [f32; 4]) -> Self {
        Self {
            render_type: RenderType::Primitive {
                primitive_type,
                parameters,
            },
            visible: true,
            color,
        }
    }

    pub fn new_rectangle(color: Color, width: f32, height: f32) -> Self {
        Self::new_primitive(PrimitiveType::Rectangle, color, [width, height, 0.0, 0.0])
    }

    pub fn new_circle(color: Color, radius: f32) -> Self {
        Self::new_primitive(PrimitiveType::Circle, color, [radius, 0.0, 0.0, 0.0])
    }

    pub fn new_line(color: Color, start: [f32; 2], end: [f32; 2]) -> Self {
        Self::new_primitive(PrimitiveType::Line, color, [start[0], start[1], end[0], end[1]])
    }

    pub fn new_texture(texture: &Texture) -> Self {
        Self {
            render_type: RenderType::Texture {
                image_data: texture.image_data.clone(),
                width: texture.width,
                height: texture.height,
            },
            color: Color::WHITE, 
            visible: true,
        }
    }
}

impl Component for Renderable {}