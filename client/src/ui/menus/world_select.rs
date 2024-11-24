use std::borrow::Borrow;

use bevy::prelude::*;

use crate::{
    lang::Locale,
    ui::{
        backgrounds::spawn_select_menu_root,
        button::{spawn_menu_button, MenuButtonAction, FONT},
        helper::{node, shadow_text, FlexColumn, NodeBundleExt},
    }, util::toggle::Toggled,
};
use bevy_simple_text_input::{TextInputBundle, TextInputSubmitEvent};

use super::MenuState;

pub fn draw_world_select(mut commands: Commands, assets: Res<AssetServer>, locale: Res<Locale>) {
    let font = assets.load(FONT);

    spawn_select_menu_root(
        &mut commands,
        // top part of world select with search bar
        |top| {
            // Select World header
            top.spawn(TextBundle::from_section(
                locale.get("select-world"),
                TextStyle {
                    font: font.clone(),
                    font_size: 35.0,
                    color: Color::WHITE,
                },
            ));

            // Text input box for world search
            top.spawn((
                WorldSelectAction::SearchWorldList,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        max_width: Val::Px(512.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                },
                TextInputBundle::default().with_text_style(TextStyle {
                    font: font.clone(),
                    font_size: 25.0,
                    color: Color::WHITE,
                }),
            ));
        },
        |_center| {},
        |bottom| {
            bottom.spawn(
                Style {
                    width: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                }.node()
            ).with_children(|parent| {
                spawn_menu_button(parent, &assets, locale.get("play-selected"), MenuButtonAction::QuitGame, Some(Val::Percent(50.0)), Toggled::Off);
                spawn_menu_button(parent, &assets, locale.get("create-world"), MenuButtonAction::QuitGame, Some(Val::Percent(50.0)), Toggled::On);
            });

            bottom.spawn(
                Style {
                    width: Val::Percent(100.0),
                    column_gap: Val::Px(10.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                }.node()
            ).with_children(|parent| {
                spawn_menu_button(parent, &assets, locale.get("edit"), MenuButtonAction::QuitGame, Some(Val::Percent(25.0)), Toggled::Off);
                spawn_menu_button(parent, &assets, locale.get("delete"), MenuButtonAction::QuitGame, Some(Val::Percent(25.0)), Toggled::Off);
                spawn_menu_button(parent, &assets, locale.get("re-create"), MenuButtonAction::QuitGame, Some(Val::Percent(25.0)), Toggled::Off);
                spawn_menu_button(parent, &assets, locale.get("back"), MenuButtonAction::GotoScreen(MenuState::Title), Some(Val::Percent(25.0)), Toggled::On);
            });
        },
    );
}

/// Actions that can be performed
/// on the world select screen.
#[derive(Component)]
pub enum WorldSelectAction {
    SearchWorldList,
}

pub fn world_select_action_handler(
    mut events: EventReader<TextInputSubmitEvent>,
    query: Query<&WorldSelectAction>,
) {
    for event in events.read() {
        if let Ok(action) = query.get(event.entity) {
            match action {
                WorldSelectAction::SearchWorldList => {
                    println!("Searching For: {}", event.value);
                }
                _ => {}
            }
        }
    }
}