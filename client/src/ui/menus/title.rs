use bevy::prelude::*;

use crate::lang::Locale;
use crate::util::toggle::Toggled;

use super::super::button::*;
use super::MenuRoot;
use super::MenuState;
use super::MAX_MENU_CONTENT_WIDTH;
use super::MENU_CONTENT_WIDTH;

pub fn draw_title(mut commands: Commands, assets: Res<AssetServer>, locale: Res<Locale>) {
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },
            ZIndex(10),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    // central panel
                    Node {
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
                ))
                .with_children(|parent| {
                    // The Title Image
                    parent.spawn((
                        ImageNode {
                            color: Color::WHITE,
                            image: assets.load("menus/minecraft.png"),
                            ..default()
                        },
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Auto,
                            ..default()
                        },
                    ));

                    // The 'Rust Edition' Image
                    // just below the title - I absolutely
                    // positioned this becuase it needs to overlap
                    // with the title image.
                    parent.spawn((
                        ImageNode {
                            color: Color::WHITE,
                            image: assets.load("menus/edition.png"),
                            ..default()
                        },
                        Node {
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
                    ));

                    parent
                        .spawn((
                            // container for menu buttons.
                            Node {
                                width: Val::Percent(60.0),
                                margin: UiRect::top(Val::Percent(5.0)),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            spawn_menu_button(
                                parent,
                                &assets,
                                locale.get("singleplayer"),
                                MenuButtonAction::GotoScreen(MenuState::WorldSelect),
                                None,
                                Toggled::On,
                            );

                            spawn_menu_button(
                                parent,
                                &assets,
                                locale.get("multiplayer"),
                                MenuButtonAction::GotoScreen(MenuState::Multiplayer),
                                None,
                                Toggled::On,
                            );

                            parent
                                .spawn((Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(10.0),
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    spawn_menu_button(
                                        parent,
                                        &assets,
                                        locale.get("options"),
                                        MenuButtonAction::GotoScreen(MenuState::Settings),
                                        Some(Val::Percent(50.0)),
                                        Toggled::On,
                                    );

                                    spawn_menu_button(
                                        parent,
                                        &assets,
                                        locale.get("quit"),
                                        MenuButtonAction::QuitGame,
                                        Some(Val::Percent(50.0)),
                                        Toggled::On,
                                    );
                                });
                        });
                });

            // copyright text, bottom right.
            parent.spawn((
                Text::new(locale.get("copyright")),
                TextFont {
                    font: assets.load(FONT),
                    font_size: 18.0,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..default()
                },
            ));

            // version, bottom left.
            parent.spawn((
                Text::new(locale.get("version")),
                TextFont {
                    font: assets.load(FONT),
                    font_size: 18.0,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
            ));
        });
}
