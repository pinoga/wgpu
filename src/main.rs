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
}

impl GraphicsContext {
    fn new(window: Arc<winit::window::Window>, instance: wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        Self {
            window,
            instance,
            surface,
            adapter,
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
