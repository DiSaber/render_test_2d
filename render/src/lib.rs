pub mod instance;
pub mod render_app;
pub mod render_pipeline;
pub mod render_state;
pub mod uniforms;

mod array_buffer;
mod vertex;

pub use {glam, wgpu, winit};

pub mod prelude {
    pub use crate::{instance::*, render_app::*, render_pipeline::*, render_state::*, uniforms::*};
}
