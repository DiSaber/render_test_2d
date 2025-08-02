use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::render_pipeline::RenderPipeline;

pub struct RenderApp<T, U>
where
    T: FnMut(Duration),
    U: FnMut(Duration),
{
    /// Initial window attributes
    window_attributes: Option<WindowAttributes>,
    /// Function that runs before the frame is drawn
    before_render: Option<T>,
    /// Function that runs after the frame is drawn
    after_render: Option<U>,
    /// The last instant the window was drawn to
    last_render: Instant,
    /// The current window and its render pipeline
    render_pipeline: Option<(Arc<Window>, RenderPipeline)>,
}

impl<T, U> RenderApp<T, U>
where
    T: FnMut(Duration),
    U: FnMut(Duration),
{
    /// Creates a new render app with update loop callbacks that are executed before and after rendering
    pub fn new(before_render: Option<T>, after_render: Option<U>) -> Self {
        Self {
            window_attributes: None,
            before_render,
            after_render,
            last_render: Instant::now(),
            render_pipeline: None,
        }
    }

    pub fn with_window_attributes(mut self, window_attributes: WindowAttributes) -> Self {
        self.window_attributes = Some(window_attributes);
        self
    }

    /// Runs the app and returns the winit event loop error if any occurs
    pub fn run_app(&mut self) -> Result<(), EventLoopError> {
        self.last_render = Instant::now(); // Set the instant right before the rendering starts

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self)
    }
}

impl<T, U> ApplicationHandler for RenderApp<T, U>
where
    T: FnMut(Duration),
    U: FnMut(Duration),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.window_attributes.clone().unwrap_or_default())
                .unwrap(),
        );

        self.render_pipeline = Some((
            window.clone(),
            pollster::block_on(RenderPipeline::new(window.clone())).unwrap(),
        ));

        window.request_redraw(); // Kick off rendering
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some((window, render_pipeline)) = &mut self.render_pipeline else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let elapsed = self.last_render.elapsed();
                self.last_render = Instant::now();

                if let Some(before_render) = &mut self.before_render {
                    before_render(elapsed);
                }

                render_pipeline.render();

                if let Some(after_render) = &mut self.after_render {
                    after_render(elapsed);
                }

                window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                render_pipeline.resize(new_size);
                window.request_redraw();
            }
            _ => (),
        }
    }
}
