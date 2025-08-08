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
}

impl App {
    /// Creates an app with the main schedules.
    pub fn new() -> Self {
        let mut world = World::new();

        world.init_resource::<MainScheduleOrder>();
        world.init_resource::<Textures>();

        world.resource_scope(|world, main_schedule_order: Mut<MainScheduleOrder>| {
            for label in main_schedule_order
                .before_pipeline_update
                .iter()
                .chain(main_schedule_order.after_pipeline_update.iter())
                .chain(main_schedule_order.startup.iter())
            {
                world.add_schedule(Schedule::new(*label));
            }
        });

        Self {
            world,
            window_attributes: None,
        }
    }

    /// Adds the given systems to the schedule.
    pub fn add_systems<M>(
        &mut self,
        label: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) {
        self.world
            .resource_mut::<Schedules>()
            .entry(label)
            .add_systems(systems);
    }

    /// Runs the app.
    pub fn run(&mut self) -> Result<(), EventLoopError> {
        self.world
            .resource_scope(|world, main_schedule_order: Mut<MainScheduleOrder>| {
                for &label in &main_schedule_order.startup {
                    let _ = world.try_run_schedule(label);
                }

                let render = |delta_time: Duration, render_pipeline: &mut RenderPipeline| {
                    world.insert_resource(DeltaTime(delta_time));

                    for &label in &main_schedule_order.before_pipeline_update {
                        let _ = world.try_run_schedule(label);
                    }

                    let update_render_state = update_render_state(render_pipeline, world);
                    render_pipeline.render(update_render_state);

                    for &label in &main_schedule_order.after_pipeline_update {
                        let _ = world.try_run_schedule(label);
                    }
                };

                RenderApp::new(render)
                    .with_window_attributes(self.window_attributes.take())
                    .run_app()
            })
    }

    /// Sets the window attributes.
    pub fn with_window_attributes(mut self, window_attributes: Option<WindowAttributes>) -> Self {
        self.window_attributes = window_attributes;
        self
    }
}

#[derive(Clone, Copy, Resource)]
pub struct DeltaTime(pub Duration);
