use super::*;
use bevy::math::UVec3;
use chunk::to_subchunk_origin;

/// An Iterator over 3d space in the world.
#[derive(Clone, Debug)]
pub struct Volume<'w> {
    reader: WorldReader<'w>,
    origin: IVec3,
    extent: IVec3,
}

impl<'w> Volume<'w> {
    pub fn new(reader: WorldReader<'w>, origin: WorldPos3, extent: IVec3) -> Self {
        Self {
            reader,
            origin,
            extent,
        }
    }

    /// Expand the volume by some amount in all directions.
    pub fn expand(self, amt: IVec3) -> Self {
        Volume {
            reader: self.reader,
            origin: self.origin - amt,
            extent: self.extent + amt,
        }
    }

    /// Compute the intersection of the volume and the chunk,
    /// returning the fragment if it exists.
    pub fn intersection(&self, subchunk: &'w SubChunk) -> Option<Fragment<'w>> {
        let min = self.origin.max(subchunk.origin);
        let max = (self.origin + self.extent).min(subchunk.origin + IVec3::splat(16));

        if max.x > min.x && max.y > min.y && max.z > min.z {
            Some(Fragment {
                subchunk,
                origin: min,
                end: max,
                curr: min,
            })
        } else {
            None
        }
    }

    /// Compute the intersection of the volume and the chunk,
    /// skipping the check to make sure they are non-overlapping.
    /// If this volume and the chunk do not overlap, the result
    /// of this operation will not be useful and my index out-of-bounds.
    fn intersection_unchecked(&self, subchunk: &'w SubChunk) -> Fragment<'w> {
        let min = self.origin.max(subchunk.origin);
        let max = (self.origin + self.extent).min(subchunk.origin + IVec3::splat(16));

        Fragment {
            subchunk,
            origin: min,
            end: max,
            curr: min,
        }
    }

    /// Returns true if this volume intersects the chunk.
    pub fn intersects(&self, subchunk: &'w SubChunk) -> bool {
        self.intersection(subchunk).is_some()
    }

    /// Get an iterator over the blocks in the volume,
    /// where each fragment is all blocks in a chunk that
    /// overlaps this volume. This means that this iterator
    /// is much faster, but no guarantees can be made about
    /// the order blocks are visited.
    pub fn fragments(self) -> Fragments<'w> {
        let start = to_subchunk_origin(self.origin);
        let end = to_subchunk_origin(start + self.extent);
        Fragments {
            start,
            end,
            curr: self.origin,
            volume: self,
        }
    }
}

/// Iterator over the fragments in a Volume,
/// where each fragment is all the blocks in
/// a given subchunk that this volume overlaps.
///
/// Only yields fragments of chunks that are in-world.
#[derive(Clone, Debug)]
pub struct Fragments<'w> {
    pub volume: Volume<'w>,

    /// Origin of the subchunk at volume origin.
    pub start: IVec3,

    /// Origin position of the subchunk at origin+extent+width.
    pub end: IVec3,

    /// Origin of last returned subchunk.
    pub curr: IVec3,
}

impl<'w> Iterator for Fragments<'w> {
    type Item = Fragment<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr.z <= self.end.z {
            while self.curr.x <= self.end.x {
                while self.curr.y <= self.end.y {
                    // Fragments<'w> will skip chunks that are out-of-world.
                    let key = self.curr;
                    self.curr.y += CHUNK_WIDTH as i32;
                    if let Some(subchunk) = self.volume.reader.get_subchunk(key) {
                        return Some(
                            self.volume
                                .intersection(subchunk)
                                .expect("Uh-Oh! thats a no no >:("),
                        );
                    }
                }
                self.curr.y = self.start.y;
                self.curr.x += CHUNK_WIDTH as i32;
            }
            self.curr.x = self.start.x;
            self.curr.z += CHUNK_WIDTH as i32;
        }

        None
    }
}

/// An iterator over 3d space in the world,
/// where all blocks in the iterator are known
/// to be in the same chunk.
#[derive(Clone, Debug)]
pub struct Fragment<'w> {
    pub subchunk: &'w SubChunk,
    pub origin: IVec3,
    pub end: IVec3,
    pub curr: IVec3,
}

impl<'w> Iterator for Fragment<'w> {
    type Item = (WorldPos3, BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.curr;
        let state = self.subchunk.get_block(pos);

        self.curr.y += 1;
        if self.curr.y > self.origin.y + self.end.y {
            self.curr.y = self.origin.y;
            self.curr.x += 1;

            if self.curr.x > self.origin.x + self.end.x {
                self.curr.x = self.origin.x;
                self.curr.z += 1;

                if self.curr.z == self.origin.z + self.end.z {
                    return None;
                }
            }
        }

        Some((pos, state))
    }
}

#[derive(Clone, Debug)]
pub struct Column<'w> {
    chunk: &'w Chunk,
    bottom: IVec3,
    top: i32,
    next: i32,

    /// The number of blocks remaining
    /// in the current subchunk until the
    /// next subchunk needs to be loaded.
    rem: i32,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use util::world_for_testing;
    use super::*;

    #[test]
    fn volume_fragments_one_subchunk() {
        let world = world_for_testing();
        let reader = world.reader();
        let volume = reader.volume(IVec3::new(0, 0, 0), IVec3::splat(CHUNK_WIDTH as i32 - 1));

        for fragment in volume.fragments() {
            for (pos, state) in fragment {
                assert_eq!(state.block, reader.get_block(pos).unwrap().block);     
            }
        }
    }

    #[test]
    fn volume_intersection() {
        let world = world_for_testing();
        let reader = world.reader();
        let volume = reader.volume(IVec3::splat(0), IVec3::splat(CHUNK_WIDTH as i32 - 1));
        let fragment = volume.intersection(reader.get_subchunk(IVec3::splat(0)).unwrap()).unwrap();
        assert_eq!(fragment.origin, IVec3::splat(0));
    }

    #[test]
    fn volume_fragments_multi_subchunk() {
        let world = world_for_testing();
        let reader = world.reader();
        let volume = reader.volume(IVec3::new(-1, -1, -1), IVec3::splat(CHUNK_WIDTH as i32 + 1));

        let mut corners: HashSet<IVec3> = HashSet::from_iter([
            IVec3::new(-1, -1, -1), 
            IVec3::splat(CHUNK_WIDTH as i32),
            IVec3::new(-1, CHUNK_WIDTH as i32, -1),
            IVec3::new(CHUNK_WIDTH as i32, -1, -1),
            IVec3::new(-1, -1, CHUNK_WIDTH as i32),
            IVec3::new(CHUNK_WIDTH as i32, CHUNK_WIDTH as i32, -1),
            IVec3::new(CHUNK_WIDTH as i32, -1, CHUNK_WIDTH as i32),
            IVec3::new(-1, CHUNK_WIDTH as i32, CHUNK_WIDTH as i32),
        ]);

        for fragment in volume.fragments() {
            eprintln!("origin: {}", fragment.origin);
            for (pos, state) in fragment {
                corners.remove(&pos);
                assert_eq!(
                    Some(state.block), 
                    reader.get_block(pos).map(|some| some.block),
                    "at: {}", pos
                );
            }
        }

        assert_eq!(0, corners.len());
    }
}
