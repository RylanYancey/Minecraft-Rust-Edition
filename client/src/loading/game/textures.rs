use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LoadTextureFiles;

#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BuildTextureAtlases;
