use bevy_ecs::component::Component;

use crate::prelude::{DenseStorageIndex, Sampler, Texture};

#[derive(Component)]
pub struct Material {
    pub texture: DenseStorageIndex<Texture>,
    pub sampler: DenseStorageIndex<Sampler>,
}

impl Material {
    pub fn new(texture: DenseStorageIndex<Texture>, sampler: DenseStorageIndex<Sampler>) -> Self {
        Self { texture, sampler }
    }
}
