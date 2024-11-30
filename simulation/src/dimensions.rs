
use bevy::prelude::*;
use bevy::app::AppLabel;
use crate::world::World;

/// Systems, Resources, and Events that
/// need to exist on all SubApps. 
pub struct DimensionPlugin;

impl Plugin for DimensionPlugin {
    fn build(&self, app: &mut App) {
        app
            // every dimension has its own world.
            .init_resource::<World>()
        ;
    }
}

#[derive(AppLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Overworld;

/// This is where you declare any Systems, Resources, or events
/// that belong to the overworld exclusively.
impl Plugin for Overworld {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(Overworld)
            // global plugins every dimension has.
            .add_plugins(DimensionPlugin)
        ;
    }
}

#[derive(AppLabel, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Nether;

/// This is where you declare any Systems, Resources, or events
/// that belong to the nether exclusively.
impl Plugin for Nether {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(Nether)
            // global plugins every dimension has.
            .add_plugins(DimensionPlugin)
        ;
    }
}
