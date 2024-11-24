use bevy::{ecs::schedule::ScheduleLabel, prelude::States};

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    TitleMenu,
    InSimulation,
    LoadingGame,
}
