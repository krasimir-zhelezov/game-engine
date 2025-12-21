use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, RenderPipeline, ShaderSource,
};
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, wgt::TextureViewDescriptor};
use wgpu::{
    Features, Instance, Limits, MemoryHints, PowerPreference, RequestAdapterOptions,
    wgt::DeviceDescriptor,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::components::camera::Camera;
use crate::components::renderable::{self, Color, PrimitiveType, RenderType, Renderable};
use crate::components::transform::{Position, Scale, Transform};
use crate::entities::entity::Entity;
use crate::systems::camera_system::CameraState;
use crate::systems::system::System;
use crate::world::WorldView;

fn create_render_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    camera_bind_group_layout: &BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader_source = ShaderSource::Wgsl(include_str!("../shader.wgsl").into());

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: shader_source,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[camera_bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_atrributes = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            shader_location: 1,
        },
    ];

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &vertex_atrributes,
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[vertex_buffer_layout],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

fn create_rectangle_verticles(
    scale: Scale,
    color: Color,
    position: Position,
) -> (Vec<f32>, Vec<u16>) {
    let verticles = vec![
        position.x - scale.x,
        position.y - scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        position.x + scale.x,
        position.y - scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        position.x + scale.x,
        position.y + scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        position.x - scale.x,
        position.y + scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    (verticles, indices)
}

fn create_circle_verticles(
    segments: u16,
    scale: Scale,
    color: Color,
    position: Position,
) -> (Vec<f32>, Vec<u16>) {
    let mut verticles = Vec::new();
    let mut indices = Vec::new();

    verticles.extend_from_slice(&[position.x, position.y, color.r, color.g, color.b, color.a]);

    for i in 0..=segments {
        let angle = 2.0 * PI * (i as f32) / (segments as f32);
        let x = position.x + (angle.cos() * scale.x);
        let y = position.y + (angle.sin() * scale.y);
        verticles.extend_from_slice(&[x, y, color.r, color.g, color.b, color.a]);

        if i < segments {
            indices.extend_from_slice(&[0, i + 1, (i + 1) % segments + 1]);
        }
    }

    (verticles, indices)
}

fn create_line_verticles() -> (Vec<f32>, Vec<u16>) {
    let verticles = vec![-0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0];

    let indices = vec![0, 1];

    (verticles, indices)
}

struct RenderBuffer {
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub vertex_count: u32,
}

pub struct RenderSystem {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    current_render_buffer: Option<RenderBuffer>,
    buffer_cache: HashMap<u32, RenderBuffer>,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
}

impl RenderSystem {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Could not get an open GPU adapter");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
                trace: Default::default(),
            })
            .await
            .expect("Failed to get device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_configuration = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_configuration);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("camera_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_buffer"),
            size: std::mem::size_of::<CameraState>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline =
            create_render_pipeline(&device, &surface_configuration, &camera_bind_group_layout);

        RenderSystem {
            window,
            instance,
            surface,
            device,
            surface_configuration,
            queue,
            render_pipeline,
            current_render_buffer: None,
            buffer_cache: HashMap::new(),
            camera_buffer,
            camera_bind_group,
        }
    }

    pub fn update_camera(&mut self, view_projection: [[f32; 4]; 4]) {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[view_projection]),
        );
    }

    pub fn create_vertex_buffer(&self, verticles: &[f32]) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(verticles),
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    pub fn create_index_buffer(&self, indices: &[u16]) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            })
    }

    pub fn setup_primitive_buffers(
        &mut self,
        entity_id: usize,
        renderable: &Renderable,
        transform: &Transform,
    ) {
        match &renderable.render_type {
            RenderType::Primitive { primitive_type, .. } => {
                let (verticles, indices) = match primitive_type {
                    PrimitiveType::Rectangle => create_rectangle_verticles(
                        transform.scale,
                        renderable.color,
                        transform.position,
                    ),
                    PrimitiveType::Circle => create_circle_verticles(
                        16,
                        transform.scale,
                        renderable.color,
                        transform.position,
                    ),
                    PrimitiveType::Line => create_line_verticles(),
                };

                self.buffer_cache.insert(
                    entity_id as u32, // ! it must not exceed 2^32-1
                    RenderBuffer {
                        vertex_buffer: Some(self.create_vertex_buffer(&verticles)),
                        index_buffer: Some(self.create_index_buffer(&indices)),
                        vertex_count: indices.len() as u32,
                    },
                );
            }
            RenderType::Texture { .. } => {
                todo!("Implement texture rendering");
            }
        }
    }

    pub fn draw(
        &mut self,
        transforms: &Vec<Option<Transform>>,
        renderables: &Vec<Option<Renderable>>,
        camera_state: &CameraState,
    ) {
        // let camera = &camera_state.main_camera;
        self.update_camera(camera_state.view_projection);

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to get current texture");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            for (id, (transform_opt, renderable_opt)) in
                transforms.iter().zip(renderables.iter()).enumerate()
            {
                if let (Some(transform), Some(renderable)) = (transform_opt, renderable_opt) {
                    self.setup_primitive_buffers(id, renderable, &transform);

                    if !renderable.visible {
                        continue;
                    }

                    if !self.buffer_cache.contains_key(&(id as u32)) {  // ! id must not exceed 2^32-1
                        self.setup_primitive_buffers(id, renderable, transform);
                    }

                    let current_render_buffer = self.buffer_cache.get(&(id as u32)).unwrap();  // ! id must not exceed 2^32-1

                    if let (Some(vertex_buffer), Some(index_buffer)) = (
                        &current_render_buffer.vertex_buffer,
                        &current_render_buffer.index_buffer,
                    ) {
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..current_render_buffer.vertex_count, 0, 0..1);
                    }
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_configuration.width = new_size.width.max(1);
        self.surface_configuration.height = new_size.height.max(1);
        self.surface
            .configure(&self.device, &self.surface_configuration);
    }
}

impl System for RenderSystem {
    fn update(&mut self, world: &mut WorldView) {
        let transforms = world.components.get_component::<Transform>();
        let renderables = world.components.get_component::<Renderable>();

        let camera_state = world.resources.get::<CameraState>().unwrap();

        self.draw(transforms, renderables, &camera_state);
    }
}
