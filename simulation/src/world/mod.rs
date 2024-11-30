
use std::{cell::Cell, collections::BTreeMap};

// Local Imports
use crate::{blocks::BlockState, math::{Vec2, Vec3}};
use bevy::prelude::Resource;
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
mod events;
mod light;

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
    pub fn get_region_mut(&mut self, at: Vec3<i32>) -> Option<&mut Region> {
        self.regions.get_mut(&at.xz().containing_region().to_i64()).map(|r| &mut**r)
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
    pub fn get_chunk_mut(&mut self, at: Vec3<i32>) -> Option<&mut Chunk> {
        let region_wrap = at.xz() & Vec2::splat(511);
        let id = (at.xz() - region_wrap).to_i64();
        let region = self.regions.get_mut(&id)?;
        let index = region_wrap / Vec2::splat(16);
        Some(region.index_mut(*index.x() as usize, *index.z() as usize))
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
    pub fn get_subchunk_mut(&mut self, at: Vec3<i32>) -> Option<&mut SubChunk> {
        if *at.y() < 0 {
            None
        } else {
            let region_wrap = at.xz() & Vec2::splat(511);
            let id = (at.xz() - region_wrap).to_i64();
            let region = self.regions.get_mut(&id)?;
            let index = region_wrap / Vec2::splat(16);
            let chunk = region.index_mut(*index.x() as usize, *index.z() as usize);
            chunk.index_mut((*at.y() / 16) as usize)
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

    #[inline]
    pub fn get_block_mut(&mut self, at: Vec3<i32>) -> Option<&mut BlockState> {
        if let Some(subchunk) = self.get_subchunk_mut(at) {
            let u = at & Vec3::splat(15);
            Some(&mut subchunk.as_slice_mut()[(u.y() + u.x() * 16 + u.z() * 256) as usize])
        } else {
            None
        }
    }

    /// Iterate regions currently in the world.
    pub fn iter_regions(&self) -> impl Iterator<Item=&Region> {
        self.regions.iter().map(|(_, region)| &**region)
    }

    /// Iterate regions in the world, mutably.
    /// Not recommended except for internal use.
    pub fn iter_regions_mut(&mut self) -> impl Iterator<Item=&mut Region> {
        self.regions.iter_mut().map(|(_, region)| &mut**region)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            regions: BTreeMap::new()
        }
    }
}
