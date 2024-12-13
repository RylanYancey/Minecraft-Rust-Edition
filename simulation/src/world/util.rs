
use chunk::{to_subchunk_origin, CHUNK_LEN};

use crate::{blocks::Light, data::registry::LocalID};

use super::*;

/// 3x3x3 world for testing purposes.
pub fn world_for_testing() -> World {
    let mut world = World::new();
    
    for z in -1..2 {
        for x in -1..2 {
            let origin = IVec2::new(x, z) * CHUNK_WIDTH as i32;
            let mut subs = Vec::new();
            for y in 0..3 {
                subs.push(subchunk_for_testing(IVec3::new(x, y, z) * CHUNK_WIDTH as i32));
            }

            world.insert(Chunk { origin, subchunks: subs });
        }
    }

    world
}

pub fn subchunk_for_testing(origin: IVec3) -> Box<SubChunk> {
    assert_eq!(to_subchunk_origin(origin), origin);        
    
    Box::new(SubChunk {
        origin,
        blocks: (0..CHUNK_LEN)
            .map(|i| BlockState {
                block: LocalID::new(i as u32),
                light: Light::ZERO,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    })
}

pub fn chunk_for_testing(origin: IVec2) -> Chunk {
    Chunk {
        origin,
        subchunks: Vec::from_iter(
            (0..8).map(|i| {
                subchunk_for_testing(IVec3::new(origin.x, i * CHUNK_WIDTH as i32, origin.y))
            }),
        ),
    }
}
