use bevy::prelude::*;

use crate::ui::loading::{LoadHintText, LoadScreenRoot, LoadingBar};

pub fn draw_game_load_screen(assets: Res<AssetServer>, mut commands: Commands) {
    const LOADING_BAR_BORDER_WIDTH: f32 = 2.0;

    commands
        .spawn((
            LoadScreenRoot,
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(82, 70, 188)),
            ZIndex(50),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
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
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("JANZEN"),
                        TextFont {
                            font: assets.load("fonts/company.ttf"),
                            font_size: 150.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new("studios"),
                        TextFont {
                            font: Handle::default(),
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(75.0)),
                            ..default()
                        },
                    ));

                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(40.0),
                                height: Val::Px(25.0),
                                border: UiRect::all(Val::Px(LOADING_BAR_BORDER_WIDTH)),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                LoadingBar,
                                Node {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    border: UiRect::all(Val::Px(LOADING_BAR_BORDER_WIDTH)),
                                    ..default()
                                },
                                BackgroundColor(Color::WHITE),
                            ));
                        });

                    parent.spawn((
                        LoadHintText,
                        Text::new(""),
                        TextFont {
                            font: Handle::default(),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
