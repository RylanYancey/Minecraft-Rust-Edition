
use std::{cell::Cell, collections::BTreeMap};

// Local Imports
use crate::{blocks::BlockState, math::{Vec2, Vec3}};
use bevy_ecs::system::Resource;
use chunk::EMPTY_SUBCHUNK;

// Exports
pub use chunk::{Chunk, SubChunk};
pub use region::Region;
pub use reader::WorldReader;

// Module Declarations //
mod chunk;
mod region;
mod reader;
mod cached;

#[derive(Resource)]
pub struct World {
    regions: BTreeMap<i64, Box<Region>>
}

impl World {
    #[inline]
    pub fn reader<'w>(&'w self) -> WorldReader<'w> {
        WorldReader::new(self)
    }

    #[inline]
    pub fn get_region(&self, at: Vec3<i32>) -> Option<&Region> {
        self.regions.get(&at.xz().containing_region().to_i64()).map(|r| &**r)
    }

    #[inline]
    pub fn get_chunk(&self, at: Vec3<i32>) -> Option<&Chunk> {
        let region_wrap = at.xz() & Vec2::splat(511);
        let id = (at.xz() - region_wrap).to_i64();
        let region = self.regions.get(&id)?;
        let index = region_wrap / Vec2::splat(16);
        Some(region.index(*index.x() as usize, *index.z() as usize))
    }

    #[inline]
    pub fn get_subchunk(&self, at: Vec3<i32>) -> Option<&SubChunk> {
        if *at.y() < 0 {
            None
        } else {
            let region_wrap = at.xz() & Vec2::splat(511);
            let id = (at.xz() - region_wrap).to_i64();
            let region = self.regions.get(&id)?;
            let index = region_wrap / Vec2::splat(16);
            let chunk = region.index(*index.x() as usize, *index.z() as usize);
            chunk.index((*at.y() / 16) as usize)
        }
    }

    #[inline]
    pub fn get_block(&self, at: Vec3<i32>) -> Option<BlockState> {
        if let Some(subchunk) = self.get_subchunk(at) {
            let u = at & Vec3::splat(15);
            Some(subchunk.as_slice()[(u.y() + u.x() * 16 + u.z() * 256) as usize])
        } else {
            None
        }
    }
}