//! Various UI Helper Functions & Bundles

use bevy::prelude::*;

// create a ui image with the texture.
pub fn ui_image(image: Handle<Image>) -> UiImage {
    UiImage {
        texture: image,
        color: Color::WHITE,
        ..default()
    }
}

/// Text with a shadow, used for ui text.
pub fn shadow_text(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str, size: f32) {
    parent.spawn(TextBundle {
        z_index: ZIndex::Local(26),
        text: Text::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: size,
                color: Color::WHITE,
            },
        ),
        ..default()
    });

    parent.spawn(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(4.0),
            top: Val::Px(4.0),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        z_index: ZIndex::Local(25),
        text: Text::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: size,
                color: Color::srgba(0.1, 0.1, 0.1, 0.5),
            },
        ),
        ..default()
    });
}

/// Helper struct for creating node bundles
/// with Flex Display and Flex Direction Columns.
pub struct FlexColumn {
    pub width: Val,
    pub height: Val,
    pub margin: UiRect,
    pub justify: JustifyContent,
    pub align: AlignItems,
}

impl FlexColumn {
    pub fn node(self) -> NodeBundle {
        self.into()
    }
}

impl Into<NodeBundle> for FlexColumn {
    fn into(self) -> NodeBundle {
        NodeBundle {
            style: Style {
                width: self.width,
                height: self.height,
                margin: self.margin,
                justify_content: self.justify,
                align_items: self.align,
                ..default()
            },
            ..default()
        }
    }
}

impl Default for FlexColumn {
    fn default() -> Self {
        Self {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            margin: UiRect::all(Val::Px(0.0)),
            justify: JustifyContent::Center,
            align: AlignItems::Center,
        }
    }
}

// a very basic node with a width and height
pub fn node(width: Val, height: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            width,
            height,
            ..default()
        },
        ..default()
    }
}

// We end up spawning alot of nodes,
// but only actually set the style,
// soooo this is a helper function
// to convert a style to a nodebundle.
pub trait NodeBundleExt {
    fn node(self) -> NodeBundle;
}

impl NodeBundleExt for Style {
    fn node(self) -> NodeBundle {
        NodeBundle {
            style: self,
            ..default()
        }
    }
}
