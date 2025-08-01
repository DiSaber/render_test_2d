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

pub struct RenderApp<'a, T, U>
where
    T: FnMut(Duration),
    U: FnMut(Duration),
{
    window: Option<Arc<Window>>,
    window_attributes: Option<WindowAttributes>,
    before_render: &'a mut T,
    after_render: &'a mut U,
    last_render: Instant,
}

impl<'a, T, U> RenderApp<'a, T, U>
where
    T: FnMut(Duration),
    U: FnMut(Duration),
{
    /// Creates a new render app with update loop callbacks that are executed before and after rendering
    pub fn new(before_render: &'a mut T, after_render: &'a mut U) -> Self {
        Self {
            window: None,
            window_attributes: None,
            before_render,
            after_render,
            last_render: Instant::now(),
        }
    }

    pub fn with_window_attributes(mut self, window_attributes: WindowAttributes) -> Self {
        self.window_attributes = Some(window_attributes);
        self
    }

    /// Runs the app and returns the winit event loop error if any occurs
    pub fn run_app(&mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)
    }
}

impl<T, U> ApplicationHandler for RenderApp<'_, T, U>
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

        self.window = Some(window.clone());

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = &self.window else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let elapsed = self.last_render.elapsed();
                self.last_render = Instant::now();

                (self.before_render)(elapsed);

                window.request_redraw();

                (self.after_render)(elapsed);
            }
            WindowEvent::Resized(_size) => {}
            _ => (),
        }
    }
}
