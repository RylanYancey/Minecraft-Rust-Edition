use crate::math::{Vec2, Vec3};
use super::Chunk;

/// A Region is a 32x32 grid of chunks (1024 chunks total), 
/// with a block width of 512 blocks.
pub struct Region {
    origin: Vec2<i32>,
    chunks: [[Chunk; 32]; 32]
}

impl Region {
    pub fn get_chunk(&self, at: Vec3<i32>) -> &Chunk {
        let local = at.xz() & Vec2::splat(511);
        self.index(
            (local.x() / 16) as usize,
            (local.z() / 16) as usize
        )
    }

    pub fn index(&self, x: usize, z: usize) -> &Chunk {
        &self.chunks[z][x]
    }

    pub fn id(&self) -> i64 {
        self.origin.to_i64()
    }
}
