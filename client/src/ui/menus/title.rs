
use bevy::{prelude::*, ui::ContentSize};

use crate::lang::Locale;
use crate::util::toggle::Toggled;

use super::MenuRoot;
use super::super::helper::*;
use super::super::button::*;
use super::MenuState;
use super::MAX_MENU_CONTENT_WIDTH;
use super::MENU_CONTENT_WIDTH;

pub fn draw_title(
    mut commands: Commands,
    assets: Res<AssetServer>,
    locale: Res<Locale>,
) {
    commands.spawn((
        MenuRoot,
        NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },
            z_index: ZIndex::Local(10),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            // central panel
            NodeBundle {
                style: Style {
                    width: MENU_CONTENT_WIDTH,
                    height: Val::Percent(100.0),
                    max_width: MAX_MENU_CONTENT_WIDTH,
                    margin: UiRect {
                        top: Val::Percent(5.0),
                        left: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(0.0),
                    },
                    align_items: AlignItems::Center,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // The Title Image
            parent.spawn((
                ImageBundle {
                    image: ui_image(assets.load("menus/minecraft.png")),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        ..default()
                    },
                    ..default()
                },
            ));

            // The 'Rust Edition' Image
            // just below the title - I absolutely
            // positioned this becuase it needs to overlap
            // with the title image. 
            parent.spawn((
                ImageBundle {
                    image: ui_image(assets.load("menus/edition.png")),
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Auto,
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            // overlap with title image.
                            top: Val::Percent(-14.0),
                            bottom: Val::Px(0.0),
                        },
                        ..default()
                    },
                    ..default()
                },
            ));

            parent.spawn((
                // container for menu buttons.
                NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        margin: UiRect::top(Val::Percent(5.0)),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Center,
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                spawn_menu_button(
                    parent, &assets, locale.get("singleplayer"), 
                    MenuButtonAction::GotoScreen(MenuState::WorldSelect), 
                    None, Toggled::On,
                );

                spawn_menu_button(
                    parent, &assets, locale.get("multiplayer"),
                    MenuButtonAction::GotoScreen(MenuState::Multiplayer),
                    None, Toggled::On,
                );

                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()  
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_menu_button(
                        parent, &assets, locale.get("options"), 
                        MenuButtonAction::GotoScreen(MenuState::Settings),
                        Some(Val::Percent(50.0)), Toggled::On,
                    );

                    spawn_menu_button(
                        parent, &assets, locale.get("quit"),
                        MenuButtonAction::QuitGame,
                        Some(Val::Percent(50.0)), Toggled::On
                    );
                });
            });
        });

        // copyright text, bottom right.
        parent.spawn((
            TextBundle {
                text: Text::from_section(
                    locale.get("copyright"), 
                    TextStyle {
                        font: assets.load(FONT), 
                        font_size: 18.0,
                        ..default()
                    }
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
        ));

        // version, bottom left.
        parent.spawn(
            TextBundle {
                text: Text::from_section(
                    locale.get("version"),
                    TextStyle {
                        font: assets.load(FONT),
                        font_size: 18.0,
                        color: Color::WHITE,
                    }
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            }
        );
    });
}
