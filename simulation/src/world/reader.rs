use arrayvec::ArrayVec;
use cached::CACHED_NEIGHBOUR_CHUNK_BOUNDARIES;
use chunk::{to_origin_local, to_subchunk_index_prewrapped, EMPTY_CHUNK};
use std::cell::Cell;

use crate::math::Dir;

use super::*;

/// A reader that caches the last
/// accessed chunk for faster access.
#[derive(Clone, Debug)]
pub struct WorldReader<'w> {
    pub world: &'w World,
    last: Cell<&'w Chunk>,
}

impl<'w> WorldReader<'w> {
    /// Get the block at this position.
    pub fn get_block(&self, pos: WorldPos3) -> Option<BlockState> {
        // the origin of the subchunk containing
        // the position and the position relative to that origin.
        let (origin, local) = to_origin_local(pos);

        // if we do not already have the right chunk,
        // attempt to get it from the world, returning
        // None if it does not exist.
        if self.last.get().origin != origin.xz() {
            self.last
                .set(self.world.chunks.get(&combine_into_u64(origin.xz()))?);
        }

        let local2 = to_subchunk_index_prewrapped(local);

        // attempt to get the subchunk from the chunk, returning None
        // if it does not exist. If the subchunk does exist, the block
        // is guaranteed to exist.
        Some(self.last.get().get_subchunk(pos.y)?.as_slice()[to_subchunk_index_prewrapped(local)])
    }

    /// Get the chunk containing this position.
    pub fn get_chunk(&self, pos: WorldPos3) -> Option<&'w Chunk> {
        // World-space origin of the chunk.
        let origin = to_chunk_origin(pos.xz());

        // if we do not already have the right chunk, search the world
        // for it, returning None if it does not exist.
        if self.last.get().origin != origin {
            self.last
                .set(self.world.chunks.get(&combine_into_u64(origin))?);
        }

        Some(self.last.get())
    }

    /// Get the subchunk containing this point.
    pub fn get_subchunk(&self, pos: WorldPos3) -> Option<&'w SubChunk> {
        self.get_chunk(pos)?.get_subchunk(pos.y)
    }

    /// If you already have the chunk origin, prefer this over get_chunk.
    pub fn get_chunk_with_origin(&self, origin: ChunkOrigin) -> Option<&'w Chunk> {
        if self.last.get().origin != origin {
            self.last
                .set(self.world.chunks.get(&combine_into_u64(origin))?)
        }

        Some(self.last.get())
    }

    /// If you already have the chunk origin, prefer this over get_chunk.
    pub fn get_subchunk_with_origin(&self, origin: SubChunkOrigin) -> Option<&'w SubChunk> {
        self.get_chunk_with_origin(origin.xz())?
            .get_subchunk(origin.y)
    }

    /// Select a volume in the world.
    pub fn volume(&self, origin: IVec3, extent: IVec3) -> Volume<'w> {
        Volume::new(self.clone(), origin, extent)
    }

    /// Get the 6 blocks that are neighbours of this block.
    pub fn neighbours(&self, pos: WorldPos3) -> Neighbours {
        const W: usize = CHUNK_WIDTH;
        let (origin, local) = to_origin_local(pos);
        let index = to_subchunk_index_prewrapped(local);
        let packed = CACHED_NEIGHBOUR_CHUNK_BOUNDARIES[index];
        let center = self.get_subchunk_with_origin(origin);

        // in the event that the center is
        // not on any of the edges, all bocks
        // are within the same subchunk.
        if packed == 0 {
            Neighbours {
                next: 0,
                values: if let Some(center) = center {
                    let center = center.as_slice();
                    [
                        center[index + 1],
                        center[index - 1],
                        center[index + W],
                        center[index - W],
                        center[index + W * W],
                        center[index - W * W],
                    ]
                } else {
                    [BlockState::default(); 6]
                },
            }
        } else {
            Neighbours {
                next: 0,
                values: {
                    // remember this is [+y, -y, +x, -x, +z, -z]
                    let mut values = [BlockState::default(); 6];

                    if let Some(center) = center {
                        let center = center.as_slice();

                        // x neighbours are in the same subchunk
                        if packed & 0b000011 == 0 {
                            values[2] = center[index + W];
                            values[3] = center[index - W];
                        // -x is in the prev chunk
                        } else if packed & 0b000010 == 0 {
                            values[2] = center[index + W];
                            values[3] = self
                                .world
                                .get_subchunk_with_origin(origin.with_x(origin.x - W as i32))
                                .map(|sub| sub.as_slice()[index + (W * (W - 1))])
                                .unwrap_or_default();
                        // +x is in the next chunk
                        } else {
                            values[2] = self
                                .world
                                .get_subchunk_with_origin(origin.with_x(origin.x + W as i32))
                                .map(|sub| sub.as_slice()[index - (W * (W - 1))])
                                .unwrap_or_default();
                            values[3] = center[index - W];
                        }

                        // y neighbours are in the same subchunk
                        if packed & 0b001100 == 0 {
                            values[0] = center[index + 1];
                            values[1] = center[index - 1];
                        // -y is in the chunk below.
                        } else if packed & 0b001000 == 0 {
                            values[0] = center[index + 1];
                            values[1] = self
                                .world
                                .get_subchunk_with_origin(origin.with_y(origin.y - W as i32))
                                .map(|sub| sub.as_slice()[index + (W - 1)])
                                .unwrap_or_default();
                        // +y is in the chunk above
                        } else {
                            values[0] = self
                                .world
                                .get_subchunk_with_origin(origin.with_y(origin.y + W as i32))
                                .map(|sub| sub.as_slice()[index - (W - 1)])
                                .unwrap_or_default();
                            values[1] = center[index - 1];
                        }

                        // z neighbours are in the same subchunk
                        if packed & 0b110000 == 0 {
                            values[4] = center[index + W * W];
                            values[5] = center[index - W * W];
                        // -z is in the previous z chunk
                        } else if packed & 0b100000 == 0 {
                            values[4] = center[index + W * W];
                            values[5] = self
                                .world
                                .get_subchunk_with_origin(origin.with_z(origin.z - W as i32))
                                .map(|sub| sub.as_slice()[index + (W * (W * (W - 1)))])
                                .unwrap_or_default();
                        // +z is in the next z chunk
                        } else {
                            values[4] = self
                                .world
                                .get_subchunk_with_origin(origin.with_z(origin.z + W as i32))
                                .map(|sub| sub.as_slice()[index - (W * (W * (W - 1)))])
                                .unwrap_or_default();
                            values[5] = center[index - W * W];
                        }
                    } else {
                        if packed & 0b000011 != 0 {
                            if packed & 0b000010 == 0 {
                                values[3] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_x(origin.x - W as i32))
                                    .map(|sub| sub.as_slice()[index + (W * (W - 1))])
                                    .unwrap_or_default();
                            } else {
                                values[2] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_x(origin.x + W as i32))
                                    .map(|sub| sub.as_slice()[index - (W * (W - 1))])
                                    .unwrap_or_default();
                            }
                        }

                        if packed & 0b001100 != 0 {
                            if packed & 0b001000 == 0 {
                                values[1] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_y(origin.y - W as i32))
                                    .map(|sub| sub.as_slice()[index + (W - 1)])
                                    .unwrap_or_default();
                            } else {
                                values[0] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_y(origin.y + W as i32))
                                    .map(|sub| sub.as_slice()[index - (W - 1)])
                                    .unwrap_or_default();
                            }
                        }

                        if packed & 0b110000 != 0 {
                            if packed & 0b100000 == 0 {
                                values[5] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_z(origin.z - W as i32))
                                    .map(|sub| sub.as_slice()[index + (W * (W * (W - 1)))])
                                    .unwrap_or_default();
                            } else {
                                values[4] = self
                                    .world
                                    .get_subchunk_with_origin(origin.with_z(origin.z + W as i32))
                                    .map(|sub| sub.as_slice()[index - (W * (W * (W - 1)))])
                                    .unwrap_or_default();
                            }
                        }
                    }

                    values
                },
            }
        }
    }
}

impl<'w> From<&'w World> for WorldReader<'w> {
    fn from(value: &'w World) -> Self {
        Self {
            world: value,
            last: Cell::new(&EMPTY_CHUNK),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Neighbours {
    /// Always in the order of
    /// UP, DOWN, EAST, WEST, NORTH, SOUTH
    values: [BlockState; 6],
    next: usize,
}

impl Neighbours {
    const DIRS: [Dir; 6] = [
        Dir::Up,
        Dir::Down,
        Dir::East,
        Dir::West,
        Dir::North,
        Dir::South,
    ];
}

impl Iterator for Neighbours {
    type Item = (Dir, BlockState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 6 {
            None
        } else {
            let result = Some((Self::DIRS[self.next], self.values[self.next]));
            self.next += 1;
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{blocks::Light, data::registry::LocalID};

    use super::util::*;
    use super::*;

    #[test]
    fn read_blocks() {
        let world = world_for_testing();
        let reader = world.reader();
        assert_eq!(
            Some(BlockState {
                block: LocalID::new(0),
                light: Light::new(0, 0, 0, 0)
            }),
            reader.get_block(IVec3::splat(0))
        );
        assert_eq!(
            Some(BlockState {
                block: LocalID::new(1),
                light: Light::new(0, 0, 0, 0)
            }),
            reader.get_block(IVec3::new(0, 1, 0))
        );
        assert_eq!(
            Some(BlockState {
                block: LocalID::new(2),
                light: Light::new(0, 0, 0, 0)
            }),
            reader.get_block(IVec3::new(0, 2, 0))
        );
        assert_eq!(
            Some(BlockState {
                block: LocalID::new(0),
                light: Light::new(0, 0, 0, 0)
            }),
            reader.get_block(IVec3::new(0, CHUNK_WIDTH as i32, 0))
        );
    }

    #[test]
    fn all_blocks() {
        let world = world_for_testing();
        let reader = world.reader();

        for z in 0..CHUNK_WIDTH as i32 {
            for x in 0..CHUNK_WIDTH as i32 {
                for y in 0..CHUNK_WIDTH as i32 {
                    let origin = IVec3::new(x, y, z);
                    let index = y + x * CHUNK_WIDTH as i32 + z * CHUNK_WIDTH as i32 * CHUNK_WIDTH as i32;
                    assert_eq!(reader.get_block(origin).unwrap().block.index() as usize, index as usize);
                }
            }
        }
    }

    #[test]
    fn get_subchunks() {
        let world = world_for_testing();
        let reader = world.reader();
        assert_eq!(
            IVec3::new(0, 0, 0),
            reader.get_subchunk(IVec3::new(4, 4, 4)).unwrap().origin
        );
    }

    #[test]
    fn get_chunks() {
        let world = world_for_testing();
        let reader = world.reader();
        assert_eq!(
            IVec2::new(0, 0),
            reader.get_chunk(IVec3::new(4, 4, 4)).unwrap().origin
        );
    }

    #[test]
    fn corner_neighbours() {
        let world = world_for_testing();
        let reader = world.reader();

        // 0 0 0 
        let origin = IVec3::new(0, 0, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // W W W
        let origin = IVec3::splat(CHUNK_WIDTH as i32 - 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // 0 W 0
        let origin = IVec3::new(0, CHUNK_WIDTH as i32 - 1, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // 0 W W
        let origin = IVec3::new(0, CHUNK_WIDTH as i32 - 1, CHUNK_WIDTH as i32 - 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // W W 0
        let origin = IVec3::new(CHUNK_WIDTH as i32 - 1, CHUNK_WIDTH as i32 - 1, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // W 0 W
        let origin = IVec3::new(CHUNK_WIDTH as i32 - 1, 0, CHUNK_WIDTH as i32 - 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // W 0 0 
        let origin = IVec3::new(CHUNK_WIDTH as i32 - 1, 0, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        // 0 0 W
        let origin = IVec3::new(0, 0, CHUNK_WIDTH as i32 - 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }
    }

    #[test]
    fn neighbour_interiors() {
        let world = world_for_testing();
        let reader = world.reader();

        let origin = IVec3::new(6, 6, 6);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }
        
        let origin = IVec3::new(4, 1, 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }

        let origin = IVec3::new(1, 1, 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            );
        }
    }

    #[test]
    fn neighbour_edge() {
        let world = world_for_testing();
        let reader = world.reader();

        let origin = IVec3::new(0, 6, 6);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(6, 6, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(6, 0, 6);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(CHUNK_WIDTH as i32 - 1, 6, 6);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(6, CHUNK_WIDTH as i32 - 1, 6);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(6, 6, CHUNK_WIDTH as i32 - 1);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
    }

    #[test]
    fn neighbour_center_out_of_bounds() {
        let world = world_for_testing();
        let reader = world.reader();
        
        let origin = IVec3::new(0, -1, 0);
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
        
        let origin = IVec3::new(-(CHUNK_WIDTH as i32 + 1), 0, -(CHUNK_WIDTH as i32));
        for (dir, state) in reader.neighbours(origin) {
            assert_eq!(
                reader.get_block(dir + origin).unwrap_or_default().block,
                state.block,
                "at: {}", dir + origin
            )
        }
    }

    #[test]
    fn neighbours_test_all_negative() {
        let world = world_for_testing();
        let reader = world.reader();

        for z in 0..CHUNK_WIDTH as i32 {
            for x in 0..CHUNK_WIDTH as i32 {
                for y in 0..CHUNK_WIDTH as i32 {
                    let origin = IVec3::new(x, y, z) + IVec3::new(-(CHUNK_WIDTH as i32), CHUNK_WIDTH as i32, 0);
                    for (dir, state) in reader.neighbours(origin) {
                        assert_eq!(
                            reader.get_block(dir + origin).unwrap_or_default().block,
                            state.block,
                            "at: {}", dir + origin
                        )
                    }
                }
            }
        }
    }
}
