
use bevy::math::IVec3;

pub struct WorldBuffer<T> {
    data: Vec<T>,
    origin: IVec3,
    extent: IVec3,
}

