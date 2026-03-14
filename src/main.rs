use std::sync::Arc;

use wgpu::{Instance, RequestAdapterOptions};

#[derive(Default)]
struct App {
    graphics_context: Option<GraphicsContext>,
}

struct GraphicsContext {
    window: Arc<winit::window::Window>,
    wgpu_instance: wgpu::Instance,
    wgpu_surface: Option<wgpu::Surface<'static>>,
    wgpu_adapter: Option<wgpu::Adapter>,
}

impl GraphicsContext {
    fn initialize_surface(&mut self) {
        self.wgpu_surface = Some(
            self.wgpu_instance
                .create_surface(self.window.clone())
                .unwrap(),
        );
    }

    fn initialize_adapter(&mut self) {
        self.wgpu_adapter = Some(
            pollster::block_on(self.wgpu_instance.request_adapter(&RequestAdapterOptions {
                compatible_surface: self.wgpu_surface.as_ref(),
                ..Default::default()
            }))
            .unwrap(),
        );
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.graphics_context.is_some() {
            self.graphics_context = Some(GraphicsContext {
                wgpu_instance: Instance::default(),
                window: Arc::new(
                    event_loop
                        .create_window(winit::window::Window::default_attributes())
                        .unwrap(),
                ),
                wgpu_adapter: None,
                wgpu_surface: None,
            })
        }

        let ctx = self.graphics_context.as_mut().unwrap();

        ctx.initialize_surface();
        ctx.initialize_adapter();
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
                return self
                    .graphics_context
                    .as_ref()
                    .unwrap()
                    .window
                    .as_ref()
                    .request_redraw();
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
