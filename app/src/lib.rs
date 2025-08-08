pub mod app;
pub mod camera;
pub mod dense_storage;
pub mod main_schedules;
pub mod material;
pub mod textures;

mod update_render_state;

pub use {bevy_ecs, bevy_transform, render};

pub mod prelude {
    pub use crate::{
        app::*, camera::*, dense_storage::*, main_schedules::*, material::*, textures::*,
    };
}
