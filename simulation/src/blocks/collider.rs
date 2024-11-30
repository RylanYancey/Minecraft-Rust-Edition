use bevy::math::bounding::Aabb3d;

#[derive(Clone)]
pub struct BlockCollider {
    pub bounds: Aabb3d,
    pub is_solid: bool,
}

