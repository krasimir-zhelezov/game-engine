use crate::{
    components::{camera::Camera, transform::Transform},
    systems::system::System,
    world::WorldView,
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

    fn calculate_view_matrix(&self, transform: &Transform) -> [[f32; 4]; 4] {
        /*
           Moves the world into the camera's perspective
        */
        let position = transform.position;
        let rotation = transform.rotation;

        let cos_angle = rotation.cos();
        let sin_angle = rotation.sin();

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
                
                break;
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
