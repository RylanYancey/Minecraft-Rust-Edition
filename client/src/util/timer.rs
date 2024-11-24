use bevy::prelude::*;

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

impl DespawnTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

pub fn despawn_timer(
    mut commands: Commands,
    mut query: Query<(&mut DespawnTimer, Entity)>,
    time: Res<Time>,
) {    
    for (mut timer, entity) in &mut query {
        timer.0.tick(time.delta());
        
        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        } 
    }
}
