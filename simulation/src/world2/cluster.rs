
use std::marker::PhantomData;

use bevy::math::IVec2;

use crate::blocks::BlockState;

use super::{Chunk, WorldPos3};

#[derive(Clone)]
pub struct Cluster2x2<'w> {
    pub(in super) chunks: [[&'w Chunk; 2]; 2],
    pub(in super) origin: IVec2,
}

#[derive(Clone)]
pub struct Cluster3x3<'w> {
    pub(in super) chunks: [[&'w Chunk; 3]; 3],
    pub(in super) origin: IVec2,
}

impl<'w> Cluster3x3<'w> {
    pub fn get_block(&self, pos: WorldPos3) -> Option<BlockState> {
        todo!()
    }
}

pub struct ClusterMut2x2<'w> {
    pub(in super) chunks: [[*mut Chunk; 2]; 2],
    pub(in super) marker: PhantomData<&'w i32>,
    pub(in super) origin: IVec2
}

pub struct ClusterMut3x3<'w> {
    pub(in super) chunks: [[*mut Chunk; 3]; 3],
    pub(in super) marker: PhantomData<&'w i32>,
    pub(in super) origin: IVec2,
}
