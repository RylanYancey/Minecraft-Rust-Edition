
use libnoise::prelude::*;

use crate::{math::Vec3, world::SpatialIter3D};

use super::hash::Permutation;

/// A NoiseMap
pub struct NoiseMap<G: Generator> {
    /// A Buffer, where the memory
    /// layout is Y-Major, then X, then Z.
    pub buffer: Vec<G::Output>,

    /// The size of the buffer in each dimension.
    pub extents: Vec3<i32>,

    /// The generator used to occupy
    /// the buffer with values.
    pub generator: G,
    
    /// The world-space position 
    /// of the values currently
    /// in the NoiseMap.
    pub origin: Vec3<i32>,

    /// Offset relative to chunk origin.
    pub offset: Vec3<i32>,

    /// Variables for the generator.
    pub fbm: Fbm,
}

impl<G: Generator> NoiseMap<G> {
    /// Create a new NoiseMap
    pub fn new(extents: Vec3<usize>, offset: Vec3<i32>, generator: G, fbm: Fbm) -> Self {
        Self {
            buffer: vec![G::Output::default(); extents.prod()],
            extents: extents.map(|n| *n as i32), 
            origin: Vec3(0, 0, 0),
            offset,
            generator, 
            fbm,
        }
    }

    /// Compute 
    pub fn compute(&mut self, perm: &Permutation, origin: Vec3<i32>) {
        self.origin = origin + self.offset;
        self.generator.fill(
            self.origin,
            self.extents,
            &self.fbm,
            &mut self.buffer,
            &perm
        );
    }
}

pub struct Fbm {
    pub octaves: u8,
    pub frequency: f32,
    pub amplitude: f32,
    pub gain: f32,
}

impl Default for Fbm {
    fn default() -> Self {
        Self {
            octaves: 1,
            frequency: 0.3,
            amplitude: 0.3,
            gain: 0.3
        }
    }
}

pub trait Generator {
    type Output: Default + Clone;
    
    /// Sample at a given position.
    fn sample(&self, pos: Vec3<i32>, fbm: &Fbm, perm: &Permutation) -> Self::Output;

    /// Fill a noisemap
    fn fill(&self, origin: Vec3<i32>, extents: Vec3<i32>, fbm: &Fbm, buffer: &mut [Self::Output], perm: &Permutation) {
        for (pos, index) in SpatialIter3D::new(origin, extents) {
            buffer[index] = self.sample(pos, fbm, perm);
        }
    }
}

