use bevy_ecs::prelude::*;

/// Defines if an entity is visible or not.
#[derive(Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum Visibility {
    /// The entity is visible.
    #[default]
    Visible,
    /// The entity is hidden.
    Hidden,
}
