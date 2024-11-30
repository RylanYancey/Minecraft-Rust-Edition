
use bevy::prelude::*;
use crate::world::World;

pub fn perform_rtick_update(
    mut world: ResMut<World>,
) {
    for region in world.iter_regions_mut() {
        for chunk in region.iter_chunks_mut() {
            if chunk.is_simulated() {
                for subchunk in chunk.iter_subchunks_mut() {
                    
                }
            }
        }
    }
}
