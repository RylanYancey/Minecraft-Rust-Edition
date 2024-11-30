use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct LoadDescriptorSets;

#[derive(ScheduleLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct OccupyRegistries;
