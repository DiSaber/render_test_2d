use std::collections::HashMap;

use bevy_ecs::{prelude::*, system::SystemState};
use bevy_transform::components::Transform;
use render::{
    glam::Mat4,
    prelude::{Instance, RenderPipeline, Uniforms, UpdateRenderState},
    wgpu::{self, SamplerDescriptor},
};

use crate::{
    camera::Camera,
    prelude::{DenseStorageIndex, Material, Sampler, Texture, Textures},
};

pub(crate) fn init(world: &mut World) {
    world.init_resource::<RenderTextures>();

    let transforms = SystemState::new(world);
    let materials = SystemState::new(world);
    world.insert_resource(RemovedInstanceComponents {
        transforms,
        materials,
    });
}

pub(crate) fn update_render_state(
    render_pipeline: &mut RenderPipeline,
    world: &mut World,
) -> UpdateRenderState {
    let mut clear_color = None;
    let uniforms = world
        .query::<(&Camera, &Transform)>()
        .single(world)
        .ok()
        .map(|(camera, transform)| {
            clear_color = Some(camera.clear_color);
            let window_size = render_pipeline.get_window_size();
            let aspect_ratio = window_size.width as f32 / window_size.height as f32;
            Uniforms::new(
                transform.compute_matrix(),
                Mat4::orthographic_rh(
                    aspect_ratio * -camera.vertical_scale * 0.5,
                    aspect_ratio * camera.vertical_scale * 0.5,
                    -camera.vertical_scale * 0.5,
                    camera.vertical_scale * 0.5,
                    camera.near_clip,
                    camera.far_clip,
                ),
            )
        });

    let mut textures = None;
    let mut texture_resource = world.resource_mut::<Textures>();
    if texture_resource.changed {
        texture_resource.changed = false;

        let mut new_textures = Vec::new();
        let mut texture_map = HashMap::new();

        for (i, texture) in &texture_resource.textures {
            texture_map.insert(i, new_textures.len() as u32);

            // In the future store the texture views to avoid re-uploading data to the gpu
            let extent = wgpu::Extent3d {
                width: texture.size.0,
                height: texture.size.1,
                depth_or_array_layers: 1,
            };
            let new_texture = render_pipeline.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            render_pipeline.write_texture(
                new_texture.as_image_copy(),
                &texture.data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(extent.width * 4),
                    rows_per_image: None,
                },
                extent,
            );
            new_textures.push(new_texture.create_view(&wgpu::TextureViewDescriptor::default()));
        }

        let mut new_samplers = Vec::new();
        let mut sampler_map = HashMap::new();

        for (i, _sampler) in &texture_resource.samplers {
            sampler_map.insert(i, new_samplers.len() as u32);

            // In the future store the samplers to avoid re-uploading data to the gpu
            new_samplers.push(render_pipeline.create_sampler(&SamplerDescriptor::default()));
        }

        textures = Some((new_textures, new_samplers));

        world.insert_resource(RenderTextures {
            textures: texture_map,
            samplers: sampler_map,
        });
    }

    let instances_changed = world
        .query_filtered::<(), Or<(Changed<Transform>, Changed<Material>)>>()
        .iter(world)
        .next()
        .is_some();
    let instances_removed = world.resource_scope(
        |world: &mut World, mut removed_instance_components: Mut<RemovedInstanceComponents>| {
            !removed_instance_components.transforms.get(world).is_empty()
                || !removed_instance_components.materials.get(world).is_empty()
        },
    );

    let mut instances = None;
    if instances_changed || instances_removed || textures.is_some() {
        world.try_resource_scope(|world, render_textures: Mut<RenderTextures>| {
            instances = Some(
                world
                    .query::<(&Transform, &Material)>()
                    .iter(world)
                    .filter_map(|(transform, material)| {
                        match (
                            render_textures.textures.get(&material.texture),
                            render_textures.samplers.get(&material.sampler),
                        ) {
                            (Some(&texture), Some(&sampler)) => {
                                Some(Instance::new(transform.compute_matrix(), texture, sampler))
                            }
                            _ => None,
                        }
                    })
                    .collect(),
            );
        });
    }

    UpdateRenderState {
        clear_color: clear_color.unwrap_or(wgpu::Color::BLACK),
        uniforms,
        instances,
        textures,
    }
}

#[derive(Default, Resource)]
struct RenderTextures {
    textures: HashMap<DenseStorageIndex<Texture>, u32>,
    samplers: HashMap<DenseStorageIndex<Sampler>, u32>,
}

#[derive(Resource)]
struct RemovedInstanceComponents {
    transforms: SystemState<RemovedComponents<'static, 'static, Transform>>,
    materials: SystemState<RemovedComponents<'static, 'static, Material>>,
}
