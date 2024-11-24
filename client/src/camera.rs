use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), IsDefaultUiCamera, MainCamera));
}
