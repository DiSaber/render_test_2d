use std::time::Duration;

use crate::{main_schedules::*, prelude::Textures, update_render_state::update_render_state};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel, system::ScheduleSystem};
use render::{
    prelude::*,
    winit::{error::EventLoopError, window::WindowAttributes},
};

pub struct App {
    world: World,
    window_attributes: Option<WindowAttributes>,
    main_schedule_order: MainScheduleOrder,
}

impl App {
    /// Creates an app with the main schedules.
    pub fn new() -> Self {
        let mut world = World::new();

        world.init_resource::<Textures>();

        Self {
            world,
            window_attributes: None,
            main_schedule_order: MainScheduleOrder::default(),
        }
    }

    /// Adds the given systems to the schedule.
    pub fn add_systems<M>(
        &mut self,
        label: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) {
        self.world
            .get_resource_or_init::<Schedules>()
            .entry(label)
            .add_systems(systems);
    }

    /// Runs the app.
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        for &label in &self.main_schedule_order.startup {
            let _ = self.world.try_run_schedule(label);
        }

        let render = |delta_time: Duration, render_pipeline: &mut RenderPipeline| {
            self.world.insert_resource(DeltaTime(delta_time));

            for &label in &self.main_schedule_order.before_state_update {
                let _ = self.world.try_run_schedule(label);
            }

            let update_render_state = update_render_state(render_pipeline, &mut self.world);
            render_pipeline.render(update_render_state);

            for &label in &self.main_schedule_order.after_state_update {
                let _ = self.world.try_run_schedule(label);
            }

            self.world.clear_trackers();
        };

        RenderApp::new(render)
            .with_window_attributes(self.window_attributes.take())
            .run_app()
    }

    /// Sets the window attributes.
    pub fn with_window_attributes(mut self, window_attributes: Option<WindowAttributes>) -> Self {
        self.window_attributes = window_attributes;
        self
    }
}

#[derive(Clone, Copy, Resource)]
pub struct DeltaTime(pub Duration);
