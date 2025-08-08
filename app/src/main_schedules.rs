use bevy_ecs::{
    prelude::*,
    schedule::{InternedScheduleLabel, ScheduleLabel},
};

/// The default schedule ran every frame before the `Update` schedule.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct PreUpdate;

/// The default schedule ran every frame.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct Update;

/// The default schedule ran every frame after the `Update` schedule. Note: This runs after the
/// render pipeline is updated.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct PostUpdate;

/// The default schedule ran once on startup before the `Startup` schedule.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct PreStartup;

/// The default schedule ran once on startup.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct Startup;

/// The default schedule ran once on startup after the `Startup` schedule.
#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct PostStartup;

/// The main schedule order that `App` uses to run systems.
#[derive(Resource)]
pub struct MainScheduleOrder {
    /// Schedules that run before the render pipeline update.
    pub before_pipeline_update: Vec<InternedScheduleLabel>,
    /// Schedules that run after the render pipeline update.
    pub after_pipeline_update: Vec<InternedScheduleLabel>,
    /// Schedules that run once on startup before anything else.
    pub startup: Vec<InternedScheduleLabel>,
}

impl Default for MainScheduleOrder {
    fn default() -> Self {
        Self {
            before_pipeline_update: vec![PreUpdate.intern(), Update.intern()],
            after_pipeline_update: vec![PostUpdate.intern()],
            startup: vec![PreStartup.intern(), Startup.intern(), PostStartup.intern()],
        }
    }
}
