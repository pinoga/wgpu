mod constants;
use std::sync::Arc;

use wgpu::{
    BlendState, ColorTargetState, ColorWrites, FragmentState, Instance, PipelineCompilationOptions,
    PipelineLayout, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RequestAdapterOptions, ShaderModule, TextureFormat, VertexState,
};

#[derive(Default)]
struct App {
    graphics: Option<GraphicsContext>,
}

struct GraphicsContext {
    window: Arc<winit::window::Window>,
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    solid_mesh_pipeline: wgpu::RenderPipeline,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl GraphicsContext {
    fn new(window: Arc<winit::window::Window>, instance: wgpu::Instance) -> Self {
        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::defaults()
            },
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        }))
        .unwrap();

        surface.configure(&device, &surface_config);

        let solid_mesh_shader =
            device.create_shader_module(wgpu::include_wgsl!("shaders/solid_mesh.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let solid_mesh_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("solid"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                buffers: &[],
                module: &solid_mesh_shader,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                entry_point: Some("vs_main"),
            },
            fragment: Some(FragmentState {
                compilation_options: PipelineCompilationOptions::default(),
                module: &solid_mesh_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
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
        });

        Self {
            solid_mesh_pipeline,
            surface_config,
            window,
            instance,
            surface,
            adapter,
            queue,
            device,
        }
    }

    pub fn draw_triangle(&self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(constants::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                    resolve_target: None,
                })],
                ..Default::default()
            });
        }
        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        frame.present();
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.graphics.is_some() {
            return;
        }

        self.graphics = Some(GraphicsContext::new(
            Arc::new(
                event_loop
                    .create_window(winit::window::Window::default_attributes())
                    .unwrap(),
            ),
            Instance::default(),
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let graphics = self.graphics.as_mut().unwrap();
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                graphics.draw_triangle();
                return graphics.window.request_redraw();
            }
            winit::event::WindowEvent::Resized(size) => {
                graphics.surface_config.height = size.height;
                graphics.surface_config.width = size.width;
                graphics
                    .surface
                    .configure(&graphics.device, &graphics.surface_config);
            }
            _ => (),
        }
    }
}

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
