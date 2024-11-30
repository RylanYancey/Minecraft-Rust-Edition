use bevy::prelude::*;

pub mod title;
pub mod world_select;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum MenuState {
    Title,
    WorldSelect,
    Multiplayer,
    Settings,
    #[default]
    None,
}

impl MenuState {
    pub fn is_panoramic(&self) -> bool {
        match *self {
            Self::Title | Self::WorldSelect | Self::Multiplayer => true,
            _ => false,
        }
    }
}

pub const MENU_CONTENT_WIDTH: Val = Val::Percent(80.0);
pub const MAX_MENU_CONTENT_WIDTH: Val = Val::Px(1024.0);

/// A Component Intended to
/// be used as the root node
/// of a Menu page. When you
/// switch between MenuStates,
/// the MenuRoot and its children
/// will be recursively destroyed.
#[derive(Component)]
pub struct MenuRoot;

/// The background images may or may not
/// be destroyed when the menus change.
#[derive(Component)]
pub struct MenuBackground;
