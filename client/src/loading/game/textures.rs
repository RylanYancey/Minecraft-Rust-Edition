
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LoadTextureFiles;

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BuildTextureAtlases;

