
use crate::blocks::{BlockID, BlockState, Light};

use super::*;

pub struct Chunk {
    origin: Vec2<i32>,
    subchunks: Vec<Box<SubChunk>>,
}

impl Chunk {
    pub fn origin(&self) -> &Vec2<i32> {
        &self.origin
    }

    /// Index the internal subchunks vector.
    pub fn index(&self, index: usize) -> Option<&SubChunk> {
        self.subchunks.get(index).map(|v| &**v)
    }

    /// get the height of the subchunks that are
    /// loaded into memory. Block locations below 0 are
    /// considered out-of-bounds, while block locations
    /// above the stored height are not - just air. 
    /// 
    /// TL;DR: Subchunks aren't loaded if they are all air.
    pub fn stored_height(&self) -> i32 {
        self.subchunks.len() as i32 * 16
    }

    pub fn get_subchunk(&self, height: i32) -> Option<&SubChunk> {
        if height > 0 && height < self.stored_height() {
            Some(&self.subchunks[(height as usize) / 16])
        } else {
            None
        }
    }
}

pub static EMPTY_SUBCHUNK: SubChunk = SubChunk {
    origin: Vec3(512 * 1000000, 0, 512 * 1000000),
    blocks: [BlockState { block: BlockID::AIR, light: Light::default()}; 4096]
};

pub struct SubChunk {
    origin: Vec3<i32>,
    blocks: [BlockState; 4096],
}

impl SubChunk {
    pub fn origin(&self) -> &Vec3<i32> {
        &self.origin
    }

    pub fn get_block(&self, at: Vec3<i32>) -> Option<BlockState> {
        let wrap = at & Vec3::splat(15);
        if at - wrap == self.origin {
            Some(self.blocks[(wrap.y() + wrap.x() * 16 + wrap.z() * 256) as usize])
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[BlockState; 4096] {
        &self.blocks
    }

    /// Returns an iterator over the blocks in the subchunk.
    /// This iterator is y-major, meaning the data is contiguous
    /// on the Y axis. This iterator, by default, returns blocks
    /// in y-order from bottom-to-top. If you want top-to-bottom,
    /// just call .rev() on the iterator.
    pub fn blocks<'b>(&'b self) -> Blocks<'b> {
        self.into_iter()
    }

    /// Returns the column at the xz coordinates relative to the subchunk.
    /// x and z must be in the range [0,16)
    /// 
    /// Column is an iterator over the column, but can also be indexed.
    pub fn column<'b>(&'b self, xz: Vec2<i32>) -> Column<'b> {
        let lower = (xz.x() * 16 + xz.z() * 256) as usize;

        Column {
            origin: xz.extend_y(*self.origin.y()),
            column: &self.blocks[lower..lower + 16],
            curr: 0,
            back: 15,
        }
    }
}

impl<'a> IntoIterator for &'a SubChunk {
    type IntoIter = Blocks<'a>;
    type Item = (Vec3<i32>, BlockState);

    fn into_iter(self) -> Self::IntoIter {
        Blocks {
            origin: self.origin,
            extent: Vec3::splat(16),
            blocks: &self.blocks,
            start: Vec3::splat(0),
            end: Vec3::splat(15),
            start_idx: 0,
            end_idx: 4095,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Blocks<'b> {
    origin: Vec3<i32>,
    extent: Vec3<i32>,
    blocks: &'b [BlockState],
    start: Vec3<i32>,
    end: Vec3<i32>,
    start_idx: usize,
    end_idx: usize,
}

impl<'b> Iterator for Blocks<'b> {
    type Item = (Vec3<i32>, BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.y() == self.extent.y() {
            self.start.1 = 0;
            self.start.0 += 1;

            if self.start.x() == self.extent.x() {
                self.start.0 = 0;
                self.start.2 += 1;

                if self.start.z() == self.extent.z() {
                    return None
                }
            }
        }

        let result = (self.origin + self.start, self.blocks[self.start_idx]);
        self.start.0 += 1;
        self.start_idx += 1;

        Some(result)
    }
}

impl<'b> DoubleEndedIterator for Blocks<'b> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end.1 == -1 {
            self.end.1 = 15;
            self.end.0 -= 1;

            if self.end.0 == -1 {
                self.end.0 = 15;
                self.end.2 -= 1;

                if self.end.2 == -1 {
                     return None
                }
            }
        }

        let result = (self.origin + self.end, self.blocks[self.end_idx]);
        self.end.0 -= 1;
        self.end_idx -= 1;
        Some(result)
    }
}

#[derive(Copy, Clone)]
pub struct Column<'b> {
    column: &'b [BlockState],
    origin: Vec3<i32>,
    curr: usize,
    back: i32,
}

impl<'b> Column<'b> {
    pub fn index(&self, index: usize) -> (Vec3<i32>, BlockState) {
        (self.origin + Vec3(0, index as i32, 0), self.column[index])
    } 
}

impl<'b> Iterator for Column<'b> {
    type Item = (Vec3<i32>, BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.column.len() {
            None
        } else {
            let result = (self.origin + Vec3(0, self.curr as i32, 0), self.column[self.curr]);
            self.curr += 1;
            Some(result)
        }
    }
}

impl<'b> DoubleEndedIterator for Column<'b> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back == -1 {
            None
        } else {
            let result = (self.origin + Vec3(0, self.back, 0), self.column[self.back as usize]);
            self.back -= 1;
            Some(result)
        }
    }
}