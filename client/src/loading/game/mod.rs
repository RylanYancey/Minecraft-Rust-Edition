use std::any::Any;

use bevy::{core::FrameCount, ecs::system::SystemState, prelude::*};
use computed::ComputeDynamicEntries;
use register::{LoadDescriptorSets, OccupyRegistries};
use setup::Setup;
use textures::{BuildTextureAtlases, LoadTextureFiles};
use crate::{ui::{loading::{LoadHintText, LoadScreenRoot, LoadingBar}, MenuState}, util::{fade::FadeOut, timer::DespawnTimer}};
use super::{loader::Loader, ready::PipelinesReady};

pub mod textures;
pub mod register;
pub mod computed;
pub mod setup;

pub fn game_loading_handler(
    world: &mut World,
    // for whatever reason, a World system
    // can't have parameters - so we have to
    // load from SystemState.
    state: &mut SystemState<(
        ResMut<GameLoader>,
        Res<PipelinesReady>,
        Query<&mut Text, With<LoadHintText>>,
        Query<&mut Style, With<LoadingBar>>,
        Query<Entity, With<LoadScreenRoot>>,
        Commands,
        ResMut<NextState<MenuState>>,
        Res<FrameCount>,
    )>
) {      
    let (mut loader, pipelines, mut load_hint, mut load_bar, mut load_screen, mut commands, mut menu_state, frame_count) =
        // look - it is technically unsafe, IF the load system
        // tries to mutate the GameLoader, but why would you do that?
        // just be a normal human being and not stupid pls 
        state.get_mut(unsafe { &mut *(world as *mut World) });
    
    // if the loader is not done loading and the
    // pipelines are ready, begin executing load systems.
    if !loader.is_done() && pipelines.is_ready() && frame_count.0 > 4 {       
        if let Some((schedule, hint)) = loader.next_stage() {
            schedule.run(world);
            load_hint.single_mut().sections[0].value = hint.clone();
            load_bar.single_mut().width = Val::Percent(loader.progress() * 100.0);
        }

        if loader.is_done() {
            commands.entity(load_screen.single_mut())
                .insert((
                    DespawnTimer::new(1.0),
                ));

            menu_state.set(MenuState::Title);
            state.apply(world);
        }
    } 
}

pub fn add_game_load_stages(
    mut loader: ResMut<GameLoader>
) {   
    loader.add_stage(Setup, "");
    loader.add_stage(LoadTextureFiles, "Loading Textures...");
    loader.add_stage(BuildTextureAtlases, "Building Texture Atlases...");
    loader.add_stage(LoadDescriptorSets, "Loading Descriptor Sets...");
    loader.add_stage(OccupyRegistries, "Occupying Registries...");
    loader.add_stage(ComputeDynamicEntries, "Computing Dynamic Blocks...");

    loader.add_systems(Setup, (
        some_slow_system, 
    ));

    loader.add_systems(LoadTextureFiles, (
        some_slow_system,
        some_slow_system 
    ));

    loader.add_systems(BuildTextureAtlases, (
        some_slow_system,
        some_slow_system,
    ));

    loader.add_systems(LoadDescriptorSets, (
        some_slow_system,
        some_slow_system,
        some_slow_system
    ));
    
    loader.add_systems(OccupyRegistries, (
        some_slow_system,
        some_slow_system,
        some_slow_system
    ));

    loader.add_systems(ComputeDynamicEntries, (
        some_slow_system,
        some_slow_system,
    ));
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameLoader(pub Loader);

fn some_slow_system(
    _assets: Res<AssetServer>,
    _loader: ResMut<GameLoader>,
) {
    std::thread::sleep(std::time::Duration::from_millis(250))
}
