//! This module provides the camera system for the 2D game engine.
//!
//! It handles the calculation of view and projection matrices required for rendering
//! the 2D scene. The system processes entities with both [`Camera`] and [`Transform`]
//! components, computes an orthographic projection matrix based on the camera's properties,
//! and updates the global [`CameraState`] resource for the renderer to consume.

use crate::{
    components::{camera::Camera, transform::Transform},
    systems::system::System,
    world::WorldView,
};

/// Global resource that holds the state of the active camera.
///
/// This state is consumed by the rendering system to apply
/// the correct view-projection matrix to the graphics pipeline.
pub struct CameraState {
    /// A copy of the currently active main camera component.
    pub main_camera: Option<Camera>,
    /// The combined 4x4 view-projection matrix.
    pub view_projection: [[f32; 4]; 4],
}

impl CameraState {
    /// Creates a new, default `CameraState`.
    ///
    /// Initializes with no active camera and an identity matrix for the view-projection.
    pub fn new() -> Self {
        Self {
            main_camera: None,
            view_projection: identity_matrix(),
        }
    }
}

/// The ECS system responsible for updating camera matrices.
///
/// `CameraSystem` iterates through entities with `Camera` and `Transform` components,
/// calculates the 2D orthographic projection and view matrices, and updates the
/// global [`CameraState`] resource.
pub struct CameraSystem;

impl CameraSystem {
    /// Creates a new `CameraSystem`.
    pub fn new() -> Self {
        Self {}
    }

    /// Calculates the combined view-projection matrix for a 2D orthographic camera.
    ///
    /// This method generates an orthographic projection matrix based on the camera's
    /// zoom and aspect ratio, then multiplies it by the view matrix derived from the
    /// camera's transform.
    ///
    /// # Arguments
    ///
    /// * `transform` - The transform component of the camera, providing position and rotation.
    /// * `zoom` - The zoom level of the camera (defines the vertical bounds).
    /// * `aspect_ratio` - The width divided by the height of the viewport.
    /// * `near_plane` - The near clipping plane distance.
    /// * `far_plane` - The far clipping plane distance.
    fn calculate_view_projection(
        &self,
        transform: &Transform,
        zoom: f32,
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> [[f32; 4]; 4] {
        let left = -aspect_ratio * zoom;
        let right = aspect_ratio * zoom;
        let bottom = -zoom;
        let top = zoom;

        // Construct the 4x4 orthographic projection matrix.
        let ortho = [
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far_plane - near_plane), 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far_plane + near_plane) / (far_plane - near_plane),
                1.0,
            ],
        ];

        let view_matrix = self.calculate_view_matrix(transform);
        let result = self.multiply_matrices(&ortho, &view_matrix);

        result
    }

    /// Multiplies two 4x4 matrices (`a` * `b`).
    ///
    /// Uses an optimized loop structure that skips operations when `a[i][k]` is zero.
    fn multiply_matrices(&self, a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0.0; 4]; 4];

        for i in 0..4 {
            for k in 0..4 {
                if a[i][k] != 0.0 {
                    for j in 0..4 {
                        result[i][j] += a[i][k] * b[k][j];
                    }
                }
            }
        }

        result
    }

    /// Calculates the 4x4 view matrix from a `Transform`.
    ///
    /// This effectively applies the inverse of the camera's translation and rotation
    /// to move the world into the camera's local perspective.
    fn calculate_view_matrix(&self, transform: &Transform) -> [[f32; 4]; 4] {
        let position = transform.position;
        let rotation = transform.rotation;

        let cos_angle = rotation.cos();
        let sin_angle = rotation.sin();

        // 2D View Matrix incorporating translation and Z-axis rotation.
        [
            [cos_angle, sin_angle, 0.0, 0.0],
            [-sin_angle, cos_angle, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [
                -position.x * cos_angle + position.y * sin_angle,
                -position.x * sin_angle - position.y * cos_angle,
                0.0,
                1.0,
            ],
        ]
    }
}

impl System for CameraSystem {
    /// Executes the camera system logic for the current frame.
    ///
    /// Finds the first entity with both a `Camera` and `Transform`, computes its
    /// view-projection matrix, and sets it as the active `CameraState` resource.
    fn update(&mut self, world: &mut WorldView) {
        let cameras = world.components.get_component::<Camera>();
        let transforms= world.components.get_component::<Transform>();

        for (camera_opt, transform_opt) in cameras.iter().zip(transforms.iter()) {
            if let (Some(camera), Some(transform)) = (camera_opt, transform_opt) {
                let view_projection = self.calculate_view_projection(
                    transform,
                    camera.zoom,
                    camera.aspect_ratio,
                    camera.near_plane,
                    camera.far_plane,
                );

                if let Some(camera_state) = world.resources.get_mut::<CameraState>() {
                    camera_state.main_camera = Some(camera.clone());
                    camera_state.view_projection = view_projection;
                }
                
                // Currently only supports one main camera, break after the first match.
                break;
            }
        }
    }
}

/// Helper function returning a standard 4x4 identity matrix.
fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
