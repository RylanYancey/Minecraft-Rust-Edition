
// imports 
use crate::{data::TagSet, math::collider::BoundingBox};

// exports
pub use id::BlockID;
pub use light::Light;
pub use state::BlockState;
pub use face::{Faces, Face, FaceRotation, Pixels};

// module declarations
mod id;
mod light;
mod state;
mod face;
mod tag;

pub struct Block {
    /// Description of the faces in a block,
    /// used for render optimization and light
    /// propogation.
    pub faces: Faces,

    /// The tags that describe this blocks' 
    /// behaviour globally.
    pub tags: TagSet,

    /// A set of colliders, and whether or not they
    /// can be moved through.
    /// 
    /// I chose to allow multiple colliders because
    /// blocks like Honey have a non-collidable collider
    /// for detecting if the player should be slowed down,
    /// and a collidable collider 1px inset. 
    /// 
    /// The bool indicates whether or not the collider
    /// can be moved through by entities.
    pub colliders: Vec<(BoundingBox, bool)>,
}