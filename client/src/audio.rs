use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiSounds {
    pub click: Handle<AudioSource>,
}

pub fn load_ui_sounds(
    mut sounds: ResMut<UiSounds>,
    assets: Res<AssetServer>
) {
    sounds.click = assets.load("sounds/ui/click.ogg");
}

