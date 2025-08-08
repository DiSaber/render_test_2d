use bevy_ecs::prelude::*;

use crate::prelude::DenseStorage;

/// Holds texture data (Rgba8).
pub struct Texture {
    pub size: (u32, u32),
    // Make this an `Option<Vec<u8>>` in the future to allow unloading from the cpu side
    pub data: Vec<u8>,
}

/// Holds sampler data.
pub struct Sampler;

/// Holds textures and samplers.
#[derive(Resource)]
pub struct Textures {
    pub(crate) textures: DenseStorage<Texture>,
    pub(crate) samplers: DenseStorage<Sampler>,
    pub(crate) changed: bool,
}

impl Textures {
    pub fn get_textures(&self) -> &DenseStorage<Texture> {
        &self.textures
    }

    pub fn get_textures_mut(&mut self) -> &mut DenseStorage<Texture> {
        self.changed = true;
        &mut self.textures
    }

    pub fn get_samplers(&self) -> &DenseStorage<Sampler> {
        &self.samplers
    }

    pub fn get_samplers_mut(&mut self) -> &mut DenseStorage<Sampler> {
        self.changed = true;
        &mut self.samplers
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            textures: DenseStorage::new(),
            samplers: DenseStorage::new(),
            changed: false,
        }
    }
}
