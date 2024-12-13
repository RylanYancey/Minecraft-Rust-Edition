use std::cell::Cell;
use crate::{blocks::BlockState, math::{Vec3, Vec3I}};
use super::{chunk::EMPTY_SUBCHUNK, Chunk, SubChunk, World};

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

    /// Returns an iterator over a column in the world, 
    /// returning `None` if the chunk is not in-world.
    pub fn column(&self, bottom: Vec3<i32>, height: usize) -> Option<Column<'w>> {
        let index = bottom.xz().map(|n| n & 15);
        let index = index.x() * 16 + index.z() * 256;
        
        Some(Column {
            chunk: self.world.get_chunk(bottom)?,
            bottom,
            height: height as i32,
            curr: 0,
            back: height as i32,
            index: index as usize,
        })
    }

    /// Returns an iterator over the neighbours of a block,
    /// for a total of 6. Does not include the block at the origin.
    pub fn neighbours(&self, at: Vec3<i32>) -> Neighbours<'w> {
        Neighbours {
            reader: self.clone(),
            origin: at,
            curr: 0,
        }
    }

    /// Returns an iterator over ALL neighbours
    /// of a block, for a total of 26. (3x3x3 area).
    /// does not include the block at the origin.
    pub fn cluster(&self, at: Vec3<i32>) -> Cluster<'w> {
        Cluster {
            reader: self.clone(),
            origin: at,
            curr: 0,
        }
    }
}

#[derive(Clone)]
pub struct Neighbours<'w> {
    reader: WorldReader<'w>,
    origin: Vec3<i32>,
    curr: usize,
}

impl<'w> Neighbours<'w> {
    /// Returns Neighbours, but only the items that are in-world.
    pub fn filter_none(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().filter_map(|(v, opt)| opt.map(|b| (v, b)))
    }

    /// Returns Neighbours, but replaces out-of-world blocks with `BlockState::default()`.
    pub fn map_default(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().map(|(v, opt)| (v, opt.unwrap_or_default()))
    }
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

#[derive(Clone)]
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

    /// Returns Cluster, but only the items that are in-world.
    pub fn filter_none(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().filter_map(|(v, opt)| opt.map(|b| (v, b)))
    }

    /// Returns Cluster, but replaces out-of-world blocks with `BlockState::default()`.
    pub fn map_default(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().map(|(v, opt)| (v, opt.unwrap_or_default()))
    }
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

/// An iterator over a column in the world.
/// By default this iterator runs bottom-up, 
/// call Column::rev() if you want top-down.
#[derive(Clone)]
pub struct Column<'w> {
    chunk: &'w Chunk,
    bottom: Vec3<i32>,
    height: i32,
    curr: i32,
    back: i32,
    index: usize,
}

impl<'w> Column<'w> {
    pub fn filter_none(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().filter_map(|(v, opt)| opt.map(|b| (v, b)))
    }

    pub fn map_default(&self) -> impl Iterator<Item=(Vec3<i32>, BlockState)> + use<'w> {
        self.clone().map(|(v, opt)| (v, opt.unwrap_or_default()))
    }
}

impl<'w> Iterator for Column<'w> {
    type Item = (Vec3<i32>, Option<BlockState>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.height {
            return None;
        }
        
        let y = self.bottom.y() + self.curr;
        self.curr += 1;
        Some((
            self.bottom.with_y(y),
            self.chunk.get_subchunk(y)
                .map(|sub| sub.as_slice()[(y & 15) as usize + self.index])
        ))
    }
}

impl<'w> DoubleEndedIterator for Column<'w> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == -1 {
            return None;
        }

        let y = self.bottom.y() + self.back;
        self.back -= 1;
        Some((
            self.bottom.with_y(y),
            self.chunk.get_subchunk(y)
                .map(|sub| sub.as_slice()[(y & 15) as usize + self.index])
        ))
    }
}
