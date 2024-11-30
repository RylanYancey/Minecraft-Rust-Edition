use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct LoadDescriptorSets;

#[derive(ScheduleLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct OccupyRegistries;
