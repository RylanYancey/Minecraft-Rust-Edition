
#![feature(variant_count)]

use audio::UiSounds;
pub use bevy::prelude::*;
use bevy::{
    core::FrameCount, diagnostic::FrameTimeDiagnosticsPlugin, window::{PresentMode, WindowTheme}
};
use camera::spawn_camera;
pub use state::GameState;

pub mod camera;
pub mod state;
pub mod ui;
pub mod audio;
pub mod util;
pub mod lang;
pub mod loading;
pub mod diagnostic;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Minecraft: Rust Edition".into(),
                        name: Some("Minecraft: Rust Edition".into()),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // gets changed back in the make_visible system
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                // linear filters cause pixelated sprites
                // to look blurry, using nearest instead.
                .set(ImagePlugin::default_nearest()),
            ui::MinecraftUiPlugin,
            util::MinecraftUtilPlugin,
            loading::MinecraftLoadingPlugin,
            diagnostic::DiagnosticsPlugin,
        ))
        .init_state::<GameState>()
        .init_resource::<audio::UiSounds>()
        .init_resource::<lang::Locale>()
        .add_systems(Startup, (spawn_camera, audio::load_ui_sounds, lang::load_locale))
        .add_systems(Update, make_visible)
        .run();
}

/// I set the 'visible' parameter in WindowPlugin to false
/// so the white screen that shows up isn't visible. This
/// makes sure the window has some rendered content before
/// being made visible.
fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}
