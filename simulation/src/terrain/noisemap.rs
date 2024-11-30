
use libnoise::prelude::*;

use crate::math::Vec3;

use super::hash::Permutation;

/// A NoiseMap
pub struct NoiseMap<T> {
    pub buffer: Vec<T>,

    /// The size of the buffer in each dimension.
    pub extents: Vec3<usize>,

    /// The generator used to occupy
    /// the buffer with values.
    pub gen: Box<dyn Generator<T>>,
    
    /// The world-space position 
    /// of the values currently
    /// in the NoiseMap.
    pub pos: Vec3<i32>,

    /// 
    pub fbm: Fbm,
}

pub struct Fbm {
    pub octaves: u8,
    pub frequency: f32,
    pub amplitude: f32,
    pub gain: f32,
}

pub trait Generator<T> {
    fn sample(&self, pos: Vec3<i32>, fbm: &Fbm, perm: &Permutation) -> T;
    fn fill(&self, pos: Vec3<i32>, extents: Vec3<usize>, fbm: &Fbm, buffer: &mut [T]) {}
}

