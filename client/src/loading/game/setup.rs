
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Eq, PartialEq, Debug, Hash, Clone)]
pub struct Setup;
