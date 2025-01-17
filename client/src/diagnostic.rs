use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
};
use iyes_perf_ui::prelude::*;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_plugins(SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Update, toggle_debug_menu);
    }
}

#[derive(Component)]
struct DiagnosticRoot;

fn toggle_debug_menu(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query: Query<Entity, With<DiagnosticRoot>>,
) {
    if input.just_pressed(KeyCode::F3) {
        if let Ok(root_entity) = query.get_single() {
            log::info!("User Closed Debug Menu");
            commands.entity(root_entity).despawn_recursive();
        } else {
            log::info!("User Opened Debug Menu");
            commands.spawn((
                DiagnosticRoot,
                PerfUiAllEntries::default(),
                ZIndex(i32::MAX),
                Node::default(),
            ));
        }
    }
}
