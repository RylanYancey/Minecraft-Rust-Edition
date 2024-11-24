use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ComputeDynamicEntries;
