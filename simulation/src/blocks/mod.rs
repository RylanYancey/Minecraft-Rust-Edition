
// imports 
use crate::{data::TagSet, math::collider::BoundingBox};
use bevy::{color::Color, math::{bounding::Aabb3d, Vec3A}};
use bevy::math::Vec3;

// exports
pub use light::Light;
pub use state::BlockState;
pub use face::{Faces, Face, FaceRotation, Pixels, FaceCoverage};
pub use collider::BlockCollider;
pub use face::Transparency;

// module declarations
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

impl Default for Block {
    fn default() -> Self {
        Self {
            faces: Faces::all(Face {
                transparent: false,
                coverage: FaceCoverage::Full,
            }),
            tags: TagSet::new(),
            colliders: vec![
                BlockCollider {
                    bounds: Aabb3d::new(Vec3::splat(0.5), Vec3::splat(0.5)),
                    is_solid: true,
                }
            ],
            emits_light: None
        }
    }
}
