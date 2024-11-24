use std::cell::Cell;

use crate::{blocks::BlockState, math::Vec3};

use super::{chunk::EMPTY_SUBCHUNK, SubChunk, World};

#[derive(Clone)]
pub struct WorldReader<'w> {
    world: &'w World,

    /// The Last Accessed Subchunk.
    /// When the world reader is created, 
    /// this points to EMPTY_SUBCHUNK.
    last: Cell<&'w SubChunk>,
}

impl<'w> WorldReader<'w> {
    pub fn new(world: &'w World) -> Self {
        Self {
            world, last: Cell::new(&EMPTY_SUBCHUNK)
        }
    }

    pub fn get_block(&self, at: Vec3<i32>) -> Option<BlockState> {
        let wrap = at & Vec3::splat(15);
        let origin = at - wrap;
        let last = self.last.get();
        let index = (wrap.y() + wrap.x() * 16 + wrap.z() * 256) as usize;

        if origin == *last.origin() {
            Some(last.as_slice()[index])
        } else {
            if let Some(subchunk) = self.world.get_subchunk(at) {
                let block = subchunk.as_slice()[index];
                self.last.set(subchunk);
                Some(block)
            } else {
                None
            }
        }
    }
}

pub struct Neighbours<'w> {
    reader: WorldReader<'w>,
    origin: Vec3<i32>,
    curr: usize,
}

impl<'w> Neighbours<'w> {
    const NEIGHBOURS: [Vec3<i32>; 6] = [
        Vec3( 0,  1,  0),
        Vec3( 0, -1,  0),
        Vec3( 1,  0,  0),
        Vec3(-1,  0,  0),
        Vec3( 0,  0,  1),
        Vec3( 0,  0, -1)
    ];
}

impl<'w> Iterator for Neighbours<'w> {
    type Item = (Vec3<i32>, Option<BlockState>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == Self::NEIGHBOURS.len() {
            None
        } else {
            let at = self.origin + Self::NEIGHBOURS[self.curr];
            Some((at, self.reader.get_block(at)))
        }
    }
}

pub struct Cluster<'w> {
    reader: WorldReader<'w>,
    origin: Vec3<i32>,
    curr: usize,
}

impl<'w> Cluster<'w> {
    const CLUSTER: [Vec3<i32>; 26] = [
        Vec3(-1, -1, -1),
        Vec3(-1,  0, -1),
        Vec3(-1,  1, -1),
        Vec3( 0, -1, -1),
        Vec3( 0,  0, -1),
        Vec3( 0,  1, -1),
        Vec3( 1, -1, -1),
        Vec3( 1,  0, -1),
        Vec3( 1,  1, -1),
        Vec3(-1, -1,  0),
        Vec3(-1,  0,  0),
        Vec3(-1,  1,  0),
        Vec3( 0, -1,  0),
        Vec3( 0,  1,  0),
        Vec3( 1, -1,  0),
        Vec3( 1,  0,  0),
        Vec3( 1,  1,  0),
        Vec3(-1, -1,  1),
        Vec3(-1,  0,  1),
        Vec3(-1,  1,  1),
        Vec3( 0, -1,  1),
        Vec3( 0,  0,  1),
        Vec3( 0,  1,  1),
        Vec3( 1, -1,  1),
        Vec3( 1,  0,  1),
        Vec3( 1,  1,  1),
    ];
}

impl<'w> Iterator for Cluster<'w> {
    type Item = (Vec3<i32>, Option<BlockState>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == Self::CLUSTER.len() {
            None
        } else {
            let at = self.origin + Self::CLUSTER[self.curr];
            Some((at, self.reader.get_block(at)))
        }
    }
}