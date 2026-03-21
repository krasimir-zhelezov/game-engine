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

use crate::components::renderable::{self, Color, PrimitiveType, RenderType, Renderable};
use crate::components::transform::{Position, Scale, Transform};
use crate::systems::camera_system::CameraState;
use crate::systems::system::System;
use crate::world::WorldView;

fn create_render_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    camera_bind_group_layout: &BindGroupLayout,
    texture_bind_group_layout: &BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader_source = ShaderSource::Wgsl(include_str!("../shader.wgsl").into());

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: shader_source,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[camera_bind_group_layout, texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_atrributes = [
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0, // position
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x4,
            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            shader_location: 1, // color
        },
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
            shader_location: 2,
        },
    ];

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
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
        0.0,
        1.0,
        position.x + scale.x,
        position.y - scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        1.0,
        1.0,
        position.x + scale.x,
        position.y + scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        1.0,
        0.0,
        position.x - scale.x,
        position.y + scale.y,
        color.r,
        color.g,
        color.b,
        color.a,
        0.0,
        0.0,
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
    pub bind_group: Arc<BindGroup>,
}

pub struct RenderSystem {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    pub texture_bind_group_layout: BindGroupLayout,
    pub default_white_texture_bind_group: Arc<BindGroup>,
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

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let default_bind_group = Arc::new(Self::create_texture_bind_group_from_bytes(
            &device,
            &queue,
            &texture_bind_group_layout,
            &[255, 255, 255, 255],
            1,
            1,
        ));

        let render_pipeline = create_render_pipeline(
            &device,
            &surface_configuration,
            &camera_bind_group_layout,
            &texture_bind_group_layout,
        );

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
            texture_bind_group_layout,
            default_white_texture_bind_group: default_bind_group,
        }
    }

    pub fn create_texture_bind_group_from_bytes(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        rgba_bytes: &[u8],
        width: u32,
        height: u32,
    ) -> BindGroup {
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Entity Texture"),
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        })
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
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
        let (verticles, indices, bind_group) = match &renderable.render_type {
            RenderType::Primitive { primitive_type, .. } => {
                let (v, i) = match primitive_type {
                    PrimitiveType::Rectangle => create_rectangle_verticles(
                        transform.scale,
                        renderable.color,
                        transform.position,
                    ),
                    // ... handle circle/line ...
                    _ => create_line_verticles(),
                };
                // Primitives use the default white texture!
                (v, i, self.default_white_texture_bind_group.clone())
            }
            RenderType::Texture {
                image_data,
                width,
                height,
            } => {
                // Assuming your enum holds these
                let (v, i) = create_rectangle_verticles(
                    transform.scale,
                    renderable.color,
                    transform.position,
                );

                let bind_group = Arc::new(RenderSystem::create_texture_bind_group_from_bytes(
                    &self.device,
                    &self.queue,
                    &self.texture_bind_group_layout,
                    image_data,
                    *width,
                    *height,
                ));

                (v, i, bind_group)
            }
        };

        self.buffer_cache.insert(
            entity_id as u32,
            RenderBuffer {
                vertex_buffer: Some(self.create_vertex_buffer(&verticles)),
                index_buffer: Some(self.create_index_buffer(&indices)),
                vertex_count: indices.len() as u32,
                bind_group, // Store it here!
            },
        );
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
                    // self.setup_primitive_buffers(id, renderable, &transform);

                    if !renderable.visible {
                        continue;
                    }

                    if !self.buffer_cache.contains_key(&(id as u32)) {
                        // ! id must not exceed 2^32-1
                        self.setup_primitive_buffers(id, renderable, transform);
                    } else {
                        let (verticles, _) = match &renderable.render_type {
                            RenderType::Primitive {
                                primitive_type,
                                parameters,
                            } => match primitive_type {
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
                            },
                            RenderType::Texture { width, height, .. } => {
                                create_rectangle_verticles(
                                    transform.scale,
                                    renderable.color,
                                    transform.position,
                                )
                            }
                        };

                        let current_render_buffer = self.buffer_cache.get(&(id as u32)).unwrap();
                        if let Some(vertex_buffer) = &current_render_buffer.vertex_buffer {
                            self.queue.write_buffer(
                                vertex_buffer,
                                0,
                                bytemuck::cast_slice(&verticles),
                            );
                        }
                    }

                    let current_render_buffer = self.buffer_cache.get(&(id as u32)).unwrap(); // ! id must not exceed 2^32-1

                    if let (Some(vertex_buffer), Some(index_buffer)) = (
                        &current_render_buffer.vertex_buffer,
                        &current_render_buffer.index_buffer,
                    ) {
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                        render_pass.set_bind_group(1, &*current_render_buffer.bind_group, &[]);

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
