use crate::blocks::BlockState;
use bevy::math::IVec2;
use bevy::math::IVec3;
use bevy::math::Vec3Swizzles;
use chunk::to_chunk_origin;
use chunk::to_subchunk_origin;
use chunk::CHUNK_WIDTH;
use cluster::Cluster2x2;
use cluster::Cluster3x3;
use cluster::ClusterMut2x2;
use cluster::ClusterMut3x3;
use std::cell::UnsafeCell;
use std::collections::BTreeMap;
use std::marker::PhantomData;

pub use chunk::Chunk;
pub use chunk::SubChunk;
pub use reader::WorldReader;
pub use volume::Volume;

mod buffer;
mod cached;
mod chunk;
mod cluster;
mod reader;
mod util;
mod volume;

/// A location relative to world-space origin.
pub type WorldPos3 = IVec3;

/// A location relative to world-space origin.
pub type WorldPos2 = IVec2;

/// A location relative to subchunk origin.
pub type LocalPos3 = IVec3;

/// A location relative to chunk origin.
pub type LocalPos2 = IVec2;

/// A coordinate which can be the origin of a chunk.
/// This means the vec must be divisible by CHUNK_WIDTH.
pub type ChunkOrigin = IVec2;

/// A coordinate which can be the origin of a subchunk.
/// This means the vec must be divisible by CHUNK_WIDTH.
pub type SubChunkOrigin = IVec3;

#[derive(Debug)]
pub struct World {
    allocator: Vec<Box<SubChunk>>,
    chunks: BTreeMap<u64, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            allocator: Vec::new(),
            chunks: BTreeMap::new(),
        }
    }

    /// Get the chunk that contains this position, if it exists.
    pub fn get_chunk(&self, pos: WorldPos3) -> Option<&Chunk> {
        self.chunks
            .get(&combine_into_u64(to_chunk_origin(pos.xz())))
    }

    /// Get the subchunk that contains this position, if it exists.
    pub fn get_subchunk(&self, pos: WorldPos3) -> Option<&SubChunk> {
        self.get_chunk(pos)?.get_subchunk(pos.y)
    }

    /// Get a chunk with a coordinate that is known to be a valid chunk origin.
    /// Unlike get_chunk, this function will not wrap the coordinates before searching.
    pub fn get_chunk_with_origin(&self, origin: ChunkOrigin) -> Option<&Chunk> {
        self.chunks.get(&combine_into_u64(origin))
    }

    /// Get a subchunk with a coordinate that is known to be a valid subchunk origin.
    /// Unlike get_subchunk, this function will not wrap the coordinates before searching.
    pub fn get_subchunk_with_origin(&self, origin: SubChunkOrigin) -> Option<&SubChunk> {
        self.get_chunk_with_origin(origin.xz())?
            .get_subchunk(origin.y)
    }

    /// Get a struct for Random Access of the World.
    pub fn reader<'w>(&'w self) -> WorldReader<'w> {
        WorldReader::from(self)
    }

    /// Insert a chunk into the world.
    pub fn insert(&mut self, chunk: Chunk) {
        let key = combine_into_u64(chunk.origin);
        if let Some(mut old) = self.chunks.insert(key, chunk) {
            while let Some(sub) = old.subchunks.pop() {
                self.allocator.push(sub);
            }
        }
    }

    /// Remove a chunk from the world, returning true
    /// if a chunk was replaced.
    pub fn remove(&mut self, pos: ChunkOrigin) -> bool {
        if let Some(mut removed) = self.chunks.remove(&combine_into_u64(pos)) {
            while let Some(sub) = removed.subchunks.pop() {
                self.allocator.push(sub);
            }

            true
        } else {
            false
        }
    }

    /// Get a 3x3 Mutable Cluster, which includes the chunk containing
    /// the position and the neighbouring 8 chunks. Returns `None` if
    /// any of the neighbouring chunks is out-of-world.
    pub fn cluster_mut_3x3<'w>(&'w mut self, pos: WorldPos3) -> Option<ClusterMut3x3<'w>> {
        use std::mem::transmute;
        const W: i32 = CHUNK_WIDTH as i32;
        let origin = to_chunk_origin(pos.xz() - IVec2::splat(W));
        unsafe {
            Some(ClusterMut3x3 {
                chunks: [
                    [
                        // [0,0], [1,0], [2,0]
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(0, 0)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W, 0)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W * 2, 0)))?,
                        ),
                    ],
                    [
                        // [0,1], [1,1], [2,1]
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(0, W)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W, W)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W * 2, W)))?,
                        ),
                    ],
                    [
                        // [0,2], [1,2], [2,2]
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(0, W * 2)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W, W * 2)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W * 2, -W * 2)))?,
                        ),
                    ],
                ],
                marker: PhantomData,
                origin,
            })
        }
    }

    /// Get a 3x3 Cluster, which includes the chunk containing the position
    /// and the neighbouring 8 chunks. Returns `None` if any of the neighbouring
    /// chunks is out-of-world.
    pub fn cluster_3x3<'w>(&'w self, pos: WorldPos3) -> Option<Cluster3x3<'w>> {
        const W: i32 = CHUNK_WIDTH as i32;
        let origin = to_chunk_origin(pos.xz() - IVec2::splat(W));
        Some(Cluster3x3 {
            chunks: [
                [
                    // [0,0], [1,0], [2,0]
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(0, 0)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W, 0)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W * 2, 0)))?,
                ],
                [
                    // [0,1], [1,1], [2,1]
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(0, W)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W, W)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W * 2, W)))?,
                ],
                [
                    // [0,2], [1,2], [2,2]
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(0, W * 2)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W, W * 2)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W * 2, -W * 2)))?,
                ],
            ],
            origin,
        })
    }

    /// Get a 2x2 Mutable Cluster, which includes the chunk containing the position and 
    /// the nearest 3 chunks. Returns `None` if any of the neighbouring chunks is out-of-world.
    pub fn cluster_mut_2x2<'w>(&'w mut self, pos: WorldPos3) -> Option<ClusterMut2x2<'w>> {
        use std::mem::transmute;
        const W: i32 = CHUNK_WIDTH as i32;
        const H: i32 = W / 2;

        let mut origin = to_chunk_origin(pos.xz());
        match (pos.x & W >= H, pos.z & W >= H) {
            (true, false) => origin -= IVec2::splat(W),
            (false, true) => origin -= IVec2::new(W, 0),
            (false, false) => origin -= IVec2::new(0, W),
            _ => {}
        }
        unsafe {
            Some(ClusterMut2x2 {
                chunks: [
                    [
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(0, 0)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W, 0)))?,
                        ),
                    ],
                    [
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(0, W)))?,
                        ),
                        transmute(
                            self.chunks
                                .get(&combine_into_u64(origin + IVec2::new(W, W)))?,
                        ),
                    ],
                ],
                origin,
                marker: PhantomData,
            })
        }
    }

    /// Get a 2x2 Cluster, which includes the chunk containing the position
    /// and the nearest 3 chunks. Returns None if any of the chunks is out-of-world.
    /// The returned Cluster is guaranteed to have all chunks in a 16-block radius
    /// of the pos.
    pub fn cluster_2x2<'w>(&'w self, pos: WorldPos3) -> Option<Cluster2x2<'w>> {
        const W: i32 = CHUNK_WIDTH as i32;
        const H: i32 = W / 2;

        let mut origin = to_chunk_origin(pos.xz());
        match (pos.x & W >= H, pos.z & W >= H) {
            (true, false) => origin -= IVec2::splat(W),
            (false, true) => origin -= IVec2::new(W, 0),
            (false, false) => origin -= IVec2::new(0, W),
            _ => {}
        }

        Some(Cluster2x2 {
            chunks: [
                [
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(0, 0)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W, 0)))?,
                ],
                [
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(0, W)))?,
                    self.chunks
                        .get(&combine_into_u64(origin + IVec2::new(W, W)))?,
                ],
            ],
            origin,
        })
    }
}

/// combines two i32s into one i64 for faster search.
const fn combine_into_u64(pos: ChunkOrigin) -> u64 {
    ((pos.x as u32 as u64) << 32) | (pos.y as u32 as u64)
}

#[cfg(test)]
mod tests {
    use super::util::*;
    use super::*;

    #[test]
    fn get_chunks() {
        let world = world_for_testing();

        for z in -1..2 {
            for x in -1..2 {
                for y in 0..3 {
                    let expected = IVec3::new(x, y, z) * CHUNK_WIDTH as i32;
                    let actual = world.get_subchunk(expected).map(|some| some.origin);
                    assert_eq!(Some(expected), actual);
                }
            }
        }
    }

    #[test]
    fn out_of_bounds() {
        let world = world_for_testing();
        assert!(world.get_chunk(IVec3::new(94, 87, 90)).is_none());
    }
}
