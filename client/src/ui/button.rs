use super::{helper, MenuState};
use crate::audio::UiSounds;
use crate::util::last::Last;
use crate::util::timer::DespawnTimer;
use crate::util::toggle::Toggled;
use bevy::{audio::PlaybackMode, prelude::*};

pub const FONT: &'static str = "fonts/main/MinecraftRegular.otf";
pub const BUTTON: &str = "menus/buttons/button.png";
pub const BUTTON_DISABLED: &str = "menus/buttons/button_disabled.png";
pub const BUTTON_HIGHLIGHTED: &str = "menus/buttons/button_highlighted.png";

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    GotoScreen(MenuState),
    QuitGame,
}

pub fn handle_menu_button_pressed(
    sounds: Res<UiSounds>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<MenuState>>,
    mut query: Query<
        (
            &Interaction,
            &MenuButtonAction,
            &mut ImageNode,
            &mut Last<Interaction>,
            &Children,
            &Toggled<Button>,
            &ButtonImages,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut child_text: Query<&mut TextColor>,
) {
    for (interaction, action, mut image, mut last, children, toggle, images) in &mut query {
        let mut set_text_color = |to: Color| {
            if let Ok(mut text) = child_text.get_mut(children[0]) {
                text.0 = to;
            }
        };

        // do nothing if the button is disabled.
        if let Toggled::Off = toggle {
            continue;
        }

        // set the button texture according to its state
        match *interaction {
            Interaction::Pressed => {
                image.image = images.disabled.clone();
                (set_text_color)(Color::srgb(0.5, 0.5, 0.5));
            }

            Interaction::Hovered => {
                image.image = images.hovered.clone();

                // perform action on release, but only if the user released while still over the button.
                if last.is(Interaction::Pressed) {
                    (set_text_color)(Color::WHITE);

                    // match statement where the actions are actually handled.
                    match action {
                        MenuButtonAction::GotoScreen(next) => {
                            log::info!("Menu Button Action Requested: {next:?}");
                            next_state.set(*next);
                        }

                        MenuButtonAction::QuitGame => {
                            log::info!("Quit Game Requested!");
                        }
                    }

                    commands.spawn((
                        DespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)),
                        AudioPlayer(sounds.click.clone()),
                        PlaybackSettings {
                            mode: PlaybackMode::Once,
                            ..default()
                        },
                    ));
                }
            }

            Interaction::None => {
                (set_text_color)(Color::WHITE);
                image.image = images.button.clone();
            }
        }

        last.set(*interaction);
    }
}

/// Buttons can be enabled and disabled.
/// A disabled (Toggleable::Off) button
/// looks like a pressed button.
pub fn handle_button_toggle(
    mut query: Query<
        (&Toggled<Button>, &mut ImageNode, &Children, &ButtonImages),
        (Changed<Toggled<Button>>, With<Button>, With<Interaction>),
    >,
    mut child_text: Query<&mut TextColor>,
) {
    for (toggled, mut image, children, images) in &mut query {
        let mut set_text_color = |to: Color| {
            if let Ok(mut text) = child_text.get_mut(children[0]) {
                text.0 = to;
            }
        };

        match toggled {
            // the button is enabled,
            // change the button into whatever
            // state is appropriate for the Interaction.
            Toggled::On => {
                (set_text_color)(Color::WHITE);
                image.image = images.button.clone();
            }

            // the button has been disabled,
            // change the button to the disabled
            // visuals.
            Toggled::Off => {
                (set_text_color)(Color::srgb(0.5, 0.5, 0.5));
                image.image = images.disabled.clone();
            }
            _ => {}
        }
    }
}

/// Spawn a Text Menu Button, like those
/// seen on the title screen.
pub fn spawn_menu_button(
    parent: &mut ChildBuilder,
    assets: &AssetServer,
    text: &str,
    action: MenuButtonAction,
    width: Option<Val>,
    enabled: Toggled<Button>,
) {
    const BUTTON_SIZE: f32 = 30.0;

    let width = width.unwrap_or(Val::Percent(100.0));
    let font = assets.load(FONT);

    parent
        .spawn((
            ButtonImages {
                button: assets.load(BUTTON),
                hovered: assets.load(BUTTON_HIGHLIGHTED),
                disabled: assets.load(BUTTON_DISABLED),
            },
            action,
            enabled,
            Last(Interaction::None),
            Button,
            Node {
                width,
                height: Val::Px(50.0),
                // align inner text horizontally and vertically
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ImageNode {
                image: assets.load(BUTTON),
                color: Color::WHITE,
                ..default()
            },
        ))
        .with_children(|parent| {
            helper::shadow_text(parent, &font, text, BUTTON_SIZE);
        });
}

#[derive(Component)]
pub struct ButtonImages {
    pub button: Handle<Image>,
    pub hovered: Handle<Image>,
    pub disabled: Handle<Image>,
}
