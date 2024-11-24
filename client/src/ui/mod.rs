use std::time::Duration;

use backgrounds::{BLUR_HIGH, BLUR_LOW};
use bevy::prelude::*;

pub mod button;
pub mod helper;
pub mod menus;
pub mod backgrounds;
pub mod loading;

use bevy_simple_text_input::TextInputPlugin;
pub use menus::{MenuRoot, MenuState};

use crate::util::despawn::despawn;
pub struct MinecraftUiPlugin;

impl Plugin for MinecraftUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MenuState>()
            .init_resource::<backgrounds::Panorama>()
            .add_plugins(TextInputPlugin)
            .add_systems(Update, (
                button::handle_menu_button_pressed, button::handle_button_toggle, 
            ))
            .add_systems(Startup, (backgrounds::load_panoramic_images,))
        // - // GAME LOAD SCREEN // - //
            .add_systems(Startup, loading::game::draw_game_load_screen)
        // - // WORLD SELECT MENU // - //
            .add_systems(OnEnter(MenuState::WorldSelect), (backgrounds::set_panorama_blur::<BLUR_HIGH>, menus::world_select::draw_world_select))
            .add_systems(OnExit(MenuState::WorldSelect), despawn::<MenuRoot>)
            .add_systems(Update, menus::world_select::world_select_action_handler.run_if(in_state(MenuState::WorldSelect)))
        // - // TITLE MENU // - //
            // behaviours:
            //  - draw panoramic background on load
            //  - un-blur panoramic background when transitioning from WorldSelect or ServerSelect
            .add_systems(on_transition(MenuState::None, MenuState::Title), backgrounds::draw_panoramic_background)
            .add_systems(on_transition(MenuState::WorldSelect, MenuState::Title), backgrounds::set_panorama_blur::<BLUR_LOW>)
            .add_systems(OnExit(MenuState::Title), despawn::<MenuRoot>)
            .add_systems(OnEnter(MenuState::Title), menus::title::draw_title);
    }
}

fn __debug_really_slow_system() {
    std::thread::sleep(Duration::from_secs(3));
}

fn on_transition<S: States>(from: S, to: S) -> OnTransition<S> {
    OnTransition { exited: from, entered: to }
}
