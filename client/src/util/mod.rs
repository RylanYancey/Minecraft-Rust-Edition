use bevy::prelude::*;

pub mod blur;
pub mod despawn;
pub mod fade;
pub mod last;
pub mod spin;
pub mod timer;
pub mod toggle;

pub struct MinecraftUtilPlugin;

impl Plugin for MinecraftUtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<blur::BlurMaterial>::default(),))
            .add_systems(Update, (timer::despawn_timer, spin::spin))
            .add_systems(
                Update,
                (
                    fade::fade_in::<BackgroundColor>,
                    fade::fade_out::<BackgroundColor>,
                ),
            );
    }
}
