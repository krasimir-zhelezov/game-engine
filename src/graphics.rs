use std::sync::Arc;
use wgpu::{RenderPipeline, ShaderSource};
use wgpu::{wgc::device::queue, wgt::DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, RequestAdapterOptions};
use wgpu::{wgt::TextureViewDescriptor, Device, Queue, RequestAdapterOptionsBase, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

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

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
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
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_configuration.width = new_size.width.max(1);
        self.surface_configuration.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_configuration);
    }

    pub fn draw(&mut self) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to get current texture");

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
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLUE), store: wgpu::StoreOp::Store
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}