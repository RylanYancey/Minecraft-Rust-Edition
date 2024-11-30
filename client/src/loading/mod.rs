use std::{any::Any, cell::Cell, marker::PhantomData};

use bevy::{
    ecs::schedule::{NodeId, ScheduleGraph, ScheduleLabel},
    prelude::*,
};

pub mod game;
pub mod loader;
pub mod ready;

use game::GameLoader;
use ready::PipelinesReady;

pub struct MinecraftLoadingPlugin;

impl Plugin for MinecraftLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            // - // PIPELINE STATUS CHECKING // - //
            // ran into some issues where I couldn't
            // start rendering the loading screen until
            // the pipeline was ready to go. This adds
            // the PipelinesReady resource that I can use
            // to detect when that occurs.
            .init_resource::<PipelinesReady>()
            .sub_app_mut(bevy::render::RenderApp) // im not 100% on what this does
            .add_systems(ExtractSchedule, ready::update_pipelines_ready);

        app
            // Systems for game asset loading and the main loading screen.
            .init_resource::<GameLoader>()
            .add_systems(Startup, game::add_game_load_stages)
            .add_systems(Update, game::game_loading_handler);
    }
}
