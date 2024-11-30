use crate::util::toggle::Toggled;
use bevy::prelude::*;

#[derive(Component, Deref)]
pub struct Spin(pub f32);

pub fn spin(time: Res<Time>, mut query: Query<(&Spin, &mut Transform, Option<&Toggled<Spin>>)>) {
    for (speed, mut transform, toggled) in &mut query {
        if toggled.is_none() || toggled.is_some_and(|v| *v == Toggled::On) {
            transform.rotation *= Quat::from_rotation_y(**speed * time.delta_secs())
        }
    }
}
