
// imports 
use crate::{data::TagSet, math::collider::BoundingBox};
use bevy::color::Color;

// exports
pub use id::BlockID;
pub use light::Light;
pub use state::BlockState;
pub use face::{Faces, Face, FaceRotation, Pixels};
pub use collider::BlockCollider;


// module declarations
mod id;
mod light;
mod state;
mod face;
mod tag;
mod collider;

pub struct Block {
    /// Description of the faces in a block,
    /// used for render optimization and light
    /// propogation.
    pub faces: Faces,

    /// The tags that describe this blocks' 
    /// behaviour globally.
    pub tags: TagSet,

    /// The set of collision boxes that
    /// describe movement through or around
    /// the block.
    pub colliders: Vec<BlockCollider>,

    /// Whether or not the block emits light.
    pub emits_light: Option<Color>,
}
