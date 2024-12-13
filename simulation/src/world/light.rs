
use super::World;
use crate::blocks::Block;
use crate::data::Registry;
use crate::math::bits::BitIterU8;
use crate::math::Dir;
use crate::math::Vec3;
use crate::math::Vec3I;
use crate::blocks::Transparency;
use crate::blocks::Light;

/// soft light updates should be called when a block that is
/// non-light emitting and has at least 1 non-transparent 
/// face is BROKEN. In this case, light changes can only
/// increase and have a limited effective range based on 
/// neighbour light values.
pub fn soft_light_update(world: &mut World, pos: Vec3I) {
    let reader = world.reader();
    let max = reader.neighbours(pos).filter_none().max_by_key(|(_, b)| b.light);

        
}

/// All components of 'extent' must be at least 3. 
/// The 'Light' must be the world (correct) light on the edges of the buffer.
/// The 'Transparency' should have a bit if the block has transparency in the direction
/// and the block in the direction has transparency in the opposite direction. 
/// Each time this function is ran, exactly 1 light update occurs on each block.
/// The return value is the number of light values that were changed.
/// The memory layout of the buffer _must_ be y-first, then x, then z.
fn buffered_light_update(buffer: &mut [(Light, Transparency)], extent: Vec3<usize>) -> usize {
    let mut changes = 0;
    for z in 1..extent.2-1 {
        let z256 = z * 256;
        for x in 1..extent.0-1 {
            let x16 = x * 16;
            let index = x16 + z256;
            let neighbours = [
                index + 1, // up 
                index - 1, // down
                index + extent.1, // east
                index - extent.1, // west
                index + extent.1 * extent.0, // north
                index - extent.1 * extent.0  // south
            ];

            for y in 1..extent.1-1 {
                let (light, transparency) = buffer[index + y];
                let mut new = light;
                for i in BitIterU8(transparency.0) {
                    // set the light value to the max of itself and the other.
                    new = new.max(buffer[neighbours[i] + y].0);
                }

                if new != light {
                    changes += 1;
                }

                buffer[index + y].0 = new;
            }
        }
    }

    changes
}

fn collect_light_buffer(
    world: &World, 
    origin: Vec3I, 
    extent: Vec3<usize>, 
    buffer: &mut [(Light, Transparency)], 
    reset: bool
) {
    let reader = world.reader();

    for z in 0..extent.z() {
        for x in 0..extent.x() {
            if let Some(column) = reader.column(Vec3(origin.x() + x as i32, origin.y(), origin.z() + z as i32), extent.y()) {
                   
            } else {
                
            }
        }
    }
}

