
use bevy::prelude::*;
use stages::TerrainStage;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc, thread::JoinHandle};
use crate::{math::Vec3, world::Chunk};
use crate::math::Vec2;
use rayon::ThreadPool;

pub mod stages;
pub mod noisemap;
pub mod worley;
pub mod hash;

#[derive(Resource)]
pub struct TerrainGenerator {
    stages: Arc<Vec<Box<dyn TerrainStage>>>,
}

pub struct TerrainWorker {
    pub handle: JoinHandle<Chunk>,
}

impl TerrainWorker {
    pub fn new(height: u32) -> Self {
        todo!()
    }

    pub fn dispatch(&mut self, at: Vec2<i32>) {
        self.handle = std::thread::spawn(|| {
            todo!()
        });
    }

    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

unsafe impl Sync for TerrainGenerator {}
unsafe impl Send for TerrainGenerator {}
