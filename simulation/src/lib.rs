#![feature(portable_simd)]

use std::collections::HashMap;

use bevy::app::SubApp;

pub type BevyEcs = bevy::prelude::World;

pub mod world;
pub mod math;
pub mod blocks;
pub mod data;
pub mod terrain;
pub mod dimensions;



