use std::sync::Arc;
use wgpu::{wgc::device::queue, wgt::DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference, RequestAdapterOptions};
use wgpu::{wgt::TextureViewDescriptor, Device, Queue, RequestAdapterOptionsBase, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

pub async fn init_graphics(window: Arc<Window>) -> Option<Graphics> {
        let instance = Instance::default();

        // let surface = Arc::new(instance.create_surface(&**window).unwrap());

        // let window: &'static Window = Box::leak(Box::new(self.window.as_ref().unwrap().clone()));
        // let surface = instance.create_surface(window).unwrap();

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
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);
        
        let gfx= Graphics {
            window,
            instance,
            surface,
            device,
            surface_configuration: surface_config,
            queue
        };

        Some(gfx)
    }

pub struct Graphics {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub device: Device,
    pub queue: Queue,
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
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            // render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.draw();
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}