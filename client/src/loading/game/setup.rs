use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Eq, PartialEq, Debug, Hash, Clone)]
pub struct Setup;
