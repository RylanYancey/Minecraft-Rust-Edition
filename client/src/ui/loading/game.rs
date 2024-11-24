
use bevy::prelude::*;

use crate::ui::loading::{LoadHintText, LoadScreenRoot, LoadingBar};

pub fn draw_game_load_screen(assets: Res<AssetServer>, mut commands: Commands) {
    const LOADING_BAR_BORDER_WIDTH: f32 = 2.0;

    commands
        .spawn((
            LoadScreenRoot,
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb_u8(82, 70, 188)),
                z_index: ZIndex::Global(50),
                ..default()
            }
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(100.0),
                        max_width: Val::Px(1024.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::horizontal(Val::Auto),
                        padding: UiRect::bottom(Val::Percent(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "JANZEN",
                        TextStyle {
                            font: assets.load("fonts/company.otf"),
                            font_size: 150.0,
                            color: Color::WHITE,
                        },
                    ));

                    parent.spawn(
                        TextBundle::from_section(
                            "studios",
                            TextStyle {
                                font: Handle::default(),
                                font_size: 50.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(75.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(40.0),
                                height: Val::Px(25.0),
                                border: UiRect::all(Val::Px(LOADING_BAR_BORDER_WIDTH)),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::WHITE),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                LoadingBar,
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0),
                                        height: Val::Percent(100.0),
                                        border: UiRect::all(Val::Px(LOADING_BAR_BORDER_WIDTH)),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::WHITE),
                                    ..default()
                                },
                            ));
                        });

                    parent.spawn((
                        LoadHintText,
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: Handle::default(),
                                font_size: 15.0,
                                color: Color::WHITE,
                            },
                        ),
                    ));
                });
        });
}

