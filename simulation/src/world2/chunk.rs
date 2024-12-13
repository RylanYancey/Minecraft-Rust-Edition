use std::ops::Shl;

use bevy::math::IVec2;

use crate::{blocks::Light, data::registry::LocalID};

use super::*;
use super::util::*;

/// Subchunks are square, so the dims of a subchunk
/// is CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_WIDTH.
///
// The value of this field _MUST_ be a power of 2.
pub const CHUNK_WIDTH: usize = 16;

/// The length of the buffer containnig blocks within a subchunk.
pub const CHUNK_LEN: usize = CHUNK_WIDTH * CHUNK_WIDTH * CHUNK_WIDTH;

#[derive(Clone, Debug)]
pub struct Chunk {
    /// The SubChunks that make up this chunk.
    /// The order of this buffer is lowest subchunk
    /// to highest, so subchunks[0] would be a subchunk
    /// at y=0, subchunk[1] at y=16, and so on. :-)
    pub(super) subchunks: Vec<Box<SubChunk>>,

    /// The origin of the chunk in world-space.
    /// This refers specifically to the xz coordinates.
    /// The 'Y' is always assumed to be 0.
    pub(super) origin: ChunkOrigin,
}

impl Chunk {
    /// Get a block, assuming that the position is within the chunks' bounds.
    /// if the position's xz is not within the chunks' xz, the result of this
    /// operation is not guaranteed to be correct.
    ///
    /// Returns None if the position is above or below the world.
    pub fn get_block(&self, pos: WorldPos3) -> Option<BlockState> {
        if pos.y < 0 {
            return None;
        }

        Some(
            self.subchunks
                .get((pos.y as usize) / CHUNK_WIDTH)?
                .get_block(pos),
        )
    }

    /// Get the subchunk within the chunk that contains the given
    /// y value, if it exists. Returns none if the value is above or below the chunk.
    pub fn get_subchunk(&self, y: i32) -> Option<&SubChunk> {
        if y < 0 {
            return None;
        }

        self.subchunks
            .get((y as usize) / CHUNK_WIDTH)
            .map(|sub| &**sub)
    }
}

pub static EMPTY_CHUNK: Chunk = Chunk {
    subchunks: Vec::new(),
    origin: IVec2 { x: i32::MAX, y: i32::MAX },
};

#[derive(Clone, Debug)]
pub struct SubChunk {
    /// Buffer that stores the blocks within a
    /// subchunk. the memory layout of this Subchunk
    /// is Y, then X, then Z. This means data is
    /// linear along the Y axis.
    pub(super) blocks: [BlockState; CHUNK_LEN],

    /// World-space origin of the subchunk, a.k.a.
    /// the coordinate with the lowest value. The
    /// first block in blocks is located at the origin,
    /// and the last block is located at origin + IVec3::splat(CHUNK_WIDTH)
    pub(super) origin: SubChunkOrigin,
}

impl SubChunk {
    pub const fn new(origin: SubChunkOrigin) -> Self {
        Self {
            blocks: [BlockState {
                block: LocalID::new(0),
                light: Light::new(0, 0, 0, 0),
            }; CHUNK_LEN],
            origin: IVec3::splat(0),
        }
    }

    pub const fn origin(&self) -> SubChunkOrigin {
        self.origin
    }

    pub const fn as_slice(&self) -> &[BlockState] {
        &self.blocks.as_slice()
    }

    pub fn as_slice_mut(&mut self) -> &mut [BlockState] {
        self.blocks.as_mut_slice()
    }

    pub const fn as_array(&self) -> &[BlockState; CHUNK_LEN] {
        &self.blocks
    }

    /// Get the block at this position, assuming
    /// that the position is inside of the subchunk.
    /// If the position is not iside the subchunk,
    /// this function may panic or return a non-useful result.
    pub fn get_block(&self, pos: WorldPos3) -> BlockState {
        self.blocks[to_subchunk_index(pos)]
    }

    pub fn get_block_mut(&mut self, pos: WorldPos3) -> &mut BlockState {
        &mut self.blocks[to_subchunk_index(pos)]
    }

    pub fn set_block(&mut self, pos: WorldPos3, state: BlockState) {
        self.blocks[to_subchunk_index(pos)] = state;
    }

    /// Returns true if the subchunk contains the position.
    pub fn contains_position(&self, pos: WorldPos3) -> bool {
        pos.x >= self.origin.x
            && pos.x < self.origin.x + (CHUNK_WIDTH as i32)
            && pos.y >= self.origin.y
            && pos.y < self.origin.y + (CHUNK_WIDTH as i32)
            && pos.z >= self.origin.z
            && pos.z < self.origin.z + (CHUNK_WIDTH as i32)
    }
}

/// Compute the index of the block at the world-space coordinates
/// within this subchunk. If the provided position is not within
/// the subchunk, the result of this operation is not guaranteed
/// to be useful.
#[inline]
pub fn to_subchunk_index(pos: WorldPos3) -> usize {
    let pos = to_local_pos(pos);
    let index = pos.y
        + pos.x * CHUNK_WIDTH as i32
        + pos.z * CHUNK_WIDTH as i32 * CHUNK_WIDTH as i32;
    index as usize
}

/// Compute the index of the block at local coordinates within this subchunk, 
/// where each component of the position is already in the range [0,CHUNK_WIDTH)
#[inline]
pub const fn to_subchunk_index_prewrapped(pos: LocalPos3) -> usize {
    (pos.y + pos.x * CHUNK_WIDTH as i32 + pos.z * CHUNK_WIDTH as i32 * CHUNK_WIDTH as i32) as usize
}

pub static EMPTY_SUBCHUNK: SubChunk = SubChunk {
    blocks: [BlockState {
        block: LocalID::new(0),
        light: Light::new(255, 0, 0, 0),
    }; CHUNK_LEN],
    origin: IVec3::splat(i32::MAX),
};

/// Convert a world position to a position within a subchunk.
/// This wraps the coordinates to the range [0, CHUNK_WIDTH)
#[inline]
pub fn to_local_pos(pos: WorldPos3) -> LocalPos3 {
    pos - to_subchunk_origin(pos)
}

/// Get the origin of the subchunk containing this position.
#[inline]
pub fn to_subchunk_origin(pos: WorldPos3) -> SubChunkOrigin {
    pos.map(|n| n & !(CHUNK_WIDTH as i32 - 1))
}

/// Get the origin of the chunk containing this position.
#[inline]
pub fn to_chunk_origin(pos: WorldPos2) -> ChunkOrigin {
    pos.map(|n| n & !(CHUNK_WIDTH as i32 - 1))
}

/// Get the origin of the containing subchunk
/// and the local position relative to that origin.
#[inline]
pub fn to_origin_local(pos: WorldPos3) -> (SubChunkOrigin, LocalPos3) {
    let origin = to_subchunk_origin(pos);
    (origin, pos - origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_subchunk_origin() {
        const W: i32 = CHUNK_WIDTH as i32;
        assert_eq!(IVec3::splat(0), to_subchunk_origin(IVec3::new(8, 4, 3)));
        assert_eq!(IVec3::new(-W, 0, -W), to_subchunk_origin(IVec3::new(-1, 1, -1)));
        assert_eq!(IVec3::new(-W, 0, -W), to_subchunk_origin(IVec3::new(-1, 0, -1)));
        assert_eq!(IVec3::new(-W, 0, 0), to_subchunk_origin(IVec3::new(-1, 0, 0)));
        assert_eq!(IVec3::splat(0), to_subchunk_origin(IVec3::splat(0)));
        assert_eq!(IVec3::new(-W, 0, -W), to_subchunk_origin(IVec3::new(-W, 0, -W)));
    }

    #[test]
    fn index_subchunk() {
        let subchunk = subchunk_for_testing(IVec3::new(0, 0, 0));
        assert_eq!(0, subchunk.get_block(IVec3::new(0, 0, 0)).block.index());
        assert_eq!(1, subchunk.get_block(IVec3::new(0, 1, 0)).block.index());
        assert_eq!(2, subchunk.get_block(IVec3::new(0, 2, 0)).block.index());
        assert_eq!(
            CHUNK_LEN as u32 - 1,
            subchunk.get_block(IVec3::splat((CHUNK_WIDTH - 1) as i32)).block.index()
        );
        assert_eq!(
            CHUNK_WIDTH as u32 - 1,
            subchunk
                .get_block(IVec3::new(0, CHUNK_WIDTH as i32 - 1, 0))
                .block
                .index()
        );
    }

    #[test]
    fn index_subchunk_negative() {
        let subchunk =
            subchunk_for_testing(IVec3::new(-(CHUNK_WIDTH as i32), 0, -(CHUNK_WIDTH as i32)));
        assert_eq!(
            0,
            subchunk
                .get_block(IVec3::new(-(CHUNK_WIDTH as i32), 0, -(CHUNK_WIDTH as i32)))
                .block
                .index()
        );
        assert_eq!(
            4,
            subchunk
                .get_block(IVec3::new(-(CHUNK_WIDTH as i32), 4, -(CHUNK_WIDTH as i32)))
                .block
                .index()
        );
    }

    #[test]
    fn get_subchunks() {
        let chunk = chunk_for_testing(IVec2::new(0, 0));
        assert_eq!(
            IVec3::new(0, 0, 0),
            chunk.get_subchunk(3).unwrap().origin
        );
        assert_eq!(
            IVec3::new(0, CHUNK_WIDTH as i32, 0),
            chunk
                .get_subchunk(CHUNK_WIDTH as i32 + 2)
                .unwrap()
                .origin
        );
    }

    #[test]
    fn get_subchunks_negative() {
        let origin = IVec2::new(-(CHUNK_WIDTH as i32), -(CHUNK_WIDTH as i32));
        let chunk = chunk_for_testing(origin);
        assert_eq!(
            IVec3::new(origin.x, 0, origin.y),
            chunk.get_subchunk(0).unwrap().origin
        );
        assert_eq!(
            IVec3::new(origin.x, CHUNK_WIDTH as i32, origin.y),
            chunk
                .get_subchunk(CHUNK_WIDTH as i32 + 2)
                .unwrap()
                .origin
        );
    }
}
