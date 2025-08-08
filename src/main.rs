use app::{
    bevy_ecs::prelude::*,
    bevy_transform::components::Transform,
    prelude::*,
    render::{glam::Vec3, wgpu, winit::window::WindowAttributes},
};

fn spawn_test_stuff(mut commands: Commands, mut textures: ResMut<Textures>) {
    commands.spawn(Camera {
        vertical_scale: 5.0,
        near_clip: -10.0,
        far_clip: 10.0,
        clear_color: wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        },
    });

    let samplers = textures.get_samplers_mut();
    let sampler = samplers.push(Sampler);

    let textures = textures.get_textures_mut();
    let red_texture = textures.push(Texture {
        size: (1, 1),
        data: vec![255, 0, 0, 255],
    });
    let blue_texture = textures.push(Texture {
        size: (1, 1),
        data: vec![0, 0, 255, 255],
    });

    commands.spawn((Transform::IDENTITY, Material::new(red_texture, sampler)));
    commands.spawn((
        Transform::from_translation(Vec3::new(1.0, 0.0, -1.0)).with_scale(Vec3::splat(2.0)),
        Material::new(blue_texture, sampler),
    ));
}

fn main() {
    let mut app = App::new().with_window_attributes(Some(
        WindowAttributes::default()
            .with_title("Render Test 2d")
            .with_resizable(true),
    ));

    app.add_systems(Startup, spawn_test_stuff);

    app.run().unwrap();
}
