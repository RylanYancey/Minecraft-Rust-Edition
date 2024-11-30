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

    pub fn index_mut(&mut self, x: usize, z: usize) -> &mut Chunk {
        &mut self.chunks[z][x]
    }

    pub fn id(&self) -> i64 {
        self.origin.to_i64()
    }

    pub fn iter_chunks(&self) -> impl Iterator<Item=&Chunk> {
        self.chunks.iter().flat_map(|map| map.iter())
    }

    pub fn iter_chunks_mut(&mut self) -> impl Iterator<Item=&mut Chunk> {
        self.chunks.iter_mut().flat_map(|map| map.iter_mut())
    }
}

