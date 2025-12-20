use crate::components::component::Component;

#[derive(Clone, Copy)]
pub struct Camera {
    pub fov: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub aspect_ratio: f32,
    pub zoom: f32,
}

impl Component for Camera {}