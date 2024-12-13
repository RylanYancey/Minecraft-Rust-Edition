use std::num::NonZero;

#[derive(Copy, Clone, Debug)]
pub struct FBM {
    pub octaves: NonZero<u8>,
    pub frequency: f32,
    pub lacunarity: f32,
    pub amplitude: f32,
    pub gain: f32,
}
