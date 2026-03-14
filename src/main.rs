use std::sync::Arc;

use wgpu::{Instance, RequestAdapterOptions};

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

        Self {
            surface_config,
            window,
            instance,
            surface,
            adapter,
            queue,
            device,
        }
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
        ))
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                return self.graphics.as_ref().unwrap().window.request_redraw();
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
