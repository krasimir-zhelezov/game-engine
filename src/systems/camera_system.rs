use crate::{
    components::{camera::Camera, transform::Transform},
    systems::system::System,
    world::WorldView
};

pub struct CameraState {
    pub main_camera: Option<Camera>,
    pub view_projection: [[f32; 4]; 4],
}

impl CameraState {
    pub fn new() -> Self {
        Self {
            main_camera: None,
            view_projection: identity_matrix(),
        }
    }
}

pub struct CameraSystem;

impl CameraSystem {
    pub fn new() -> Self {
        Self {}
    }

    fn calculate_view_projection(&self, transform: &Transform, zoom: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> [[f32; 4]; 4] {
        // Create orthographic projection matrix
        let left = -aspect_ratio * zoom;
        let right = aspect_ratio * zoom;
        let bottom = -zoom;
        let top = zoom;

        println!("Camera bounds: left={}, right={}, bottom={}, top={}", left, right, bottom, top);

        // Correct orthographic projection matrix for 2D
        let ortho = [
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far_plane - near_plane), 0.0], // Note: negative for correct depth
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far_plane + near_plane) / (far_plane - near_plane),
                1.0,
            ],
        ];

        println!("Ortho matrix: {:?}", ortho);

        // For 2D, use identity view matrix
        let view_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let result = self.multiply_matrices(&ortho, &view_matrix);
        println!("Final view_projection: {:?}", result);
        result
    }

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
}

impl System for CameraSystem {
    fn update(&mut self, world: &mut WorldView) {

        let cameras = world.components.get_entities_with_component::<Transform, Camera>();

        // println!("Found {} cameras", cameras.len());
        
        if let Some((_, transform, camera)) = cameras.into_iter().next() {
            // println!("Camera: pos=({}, {}), zoom={}, aspect={}", 
                // transform.position.x, transform.position.y, camera.zoom, camera.aspect_ratio);
            let view_projection = self.calculate_view_projection(
                transform, 
                camera.zoom, 
                camera.aspect_ratio, 
                camera.near_plane, 
                camera.far_plane
            );

            if let Some(camera_state) = world.resources.get_mut::<CameraState>() {
                camera_state.main_camera = Some(camera.clone());
                camera_state.view_projection = view_projection;

                println!("View projection matrix updated");
            }
        }
    }
}

fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}