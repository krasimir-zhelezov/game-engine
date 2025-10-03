use std::f32::consts::PI;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, RenderPipeline, ShaderSource};
use wgpu::{wgt::DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, RequestAdapterOptions};
use wgpu::{wgt::TextureViewDescriptor, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use crate::components::{Color, PrimitiveType, RenderType, Renderable};

pub async fn init_graphics(window: Arc<Window>) -> Option<Graphics> {
    let instance = Instance::default();

    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface)
    })
    .await
    .expect("Could not get an open GPU adapter");

     let (device, queue) = adapter.request_device(
        &DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: MemoryHints::Performance,
            trace: Default::default(),
        }
    )
    .await
    .expect("Failed to get device");

    let size = window.inner_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let surface_configuration = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &surface_configuration);

    let render_pipeline = create_render_pipeline(&device, &surface_configuration);
        
    let gfx= Graphics {
        window,
        instance,
        surface,
        device,
        surface_configuration,
        queue,
        render_pipeline
    };

    Some(gfx)
}

fn create_render_pipeline(device: &Device, config: &SurfaceConfiguration) -> wgpu::RenderPipeline {
    let shader_source = ShaderSource::Wgsl(include_str!("shader.wgsl").into());

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: shader_source,
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
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

fn create_rectangle_verticles(scale: [f32; 2], color: &Color, position:  [f32; 2]) -> (Vec<f32>, Vec<u16>) {

    let verticles = vec![
        position[0] - scale[0], position[1] - scale[1],     color.r, color.g, color.b, color.a,
        position[0] + scale[0], position[1] - scale[1],      color.r, color.g, color.b, color.a,
        position[0] + scale[0], position[1] + scale[1],       color.r, color.g, color.b, color.a,
        position[0] - scale[0], position[1] + scale[1],      color.r, color.g, color.b, color.a,
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    (verticles, indices)
}

fn create_circle_verticles(segments: u16, scale: [f32; 2]) -> (Vec<f32>, Vec<u16>) {
    let mut verticles = Vec::new();
    let mut indices = Vec::new();

    verticles.extend_from_slice(&[0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);

    for i in 0..=segments {
        let angle = 2.0 * PI * (i as f32) / (segments as f32);
        let x = angle.cos() * scale[0];
        let y = angle.sin() * scale[1];
        verticles.extend_from_slice(&[x, y, 0.0, 0.0, 1.0, 1.0]);

        if i < segments {
            indices.extend_from_slice(&[0, i + 1, (i + 1) % segments + 1]);
        }
    }

    (verticles, indices)
}

fn create_line_verticles() -> (Vec<f32>, Vec<u16>) {
    let verticles = vec![
        -0.5, 0.0,     1.0, 0.0, 0.0, 1.0,
        0.5, 0.0,      1.0, 0.0, 0.0, 1.0,
    ];

    let indices = vec![0, 1];

    (verticles, indices)
}

pub struct Graphics {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline
}

impl Graphics {
    pub fn create_vertex_buffer(&self, verticles: &[f32]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(verticles),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn create_index_buffer(&self, indices: &[u16]) -> Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    pub fn setup_primitive_buffers(&self, renderable: &mut Renderable) {
        match &renderable.render_type {
            RenderType::Primitive { primitive_type, .. } => {
                let (verticles, indices) = match primitive_type {
                    PrimitiveType::Rectangle => create_rectangle_verticles(renderable.transform.scale, &renderable.color, renderable.transform.position),
                    PrimitiveType::Circle => create_circle_verticles(16, renderable.transform.scale),
                    PrimitiveType::Line => create_line_verticles(),
                };

                renderable.vertex_buffer = Some(self.create_vertex_buffer(&verticles));
                renderable.index_buffer = Some(self.create_index_buffer(&indices));
                renderable.vertex_count = indices.len() as u32;
            },
            RenderType::Texture { .. } => {
                todo!("Implement texture rendering");
            }
        }   
    }

    pub fn draw_renderables(&mut self, renderables: &mut[&mut Renderable]) {
        // println!("Rendering {} objects", renderables.len());

        let frame = self.surface.get_current_texture().expect("Failed to get current texture");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
                label: None, 
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { 
                    view: &view, 
                    depth_slice: None, 
                    resolve_target: None, 
                    ops: wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), 
                        store: wgpu::StoreOp::Store 
                    }
                })], 
                depth_stencil_attachment: None, 
                timestamp_writes: None, 
                occlusion_query_set: None 
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for mut renderable in renderables {
                self.setup_primitive_buffers(&mut renderable);

                if !renderable.visible {
                    continue;
                }

                if let (Some(vertex_buffer), Some(index_buffer)) = (&renderable.vertex_buffer, &renderable.index_buffer) {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..renderable.vertex_count, 0, 0..1);
                }
            }
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_configuration.width = new_size.width.max(1);
        self.surface_configuration.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_configuration);
    }
}