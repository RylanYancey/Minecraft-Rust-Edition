//! Various UI Helper Functions & Bundles

use bevy::prelude::*;

// create a ui image with the texture.
pub fn ui_image(image: Handle<Image>) -> ImageNode {
    ImageNode {
        image,
        color: Color::WHITE,
        ..default()
    }
}

/// Text with a shadow, used for ui text.
pub fn shadow_text(parent: &mut ChildBuilder, font: &Handle<Font>, text: &str, size: f32) {
    parent.spawn((
        ZIndex(26),
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(Color::WHITE),
    ));

    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(4.0),
            top: Val::Px(4.0),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ZIndex(25),
        Text::new(text),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
    ));
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
    pub fn node(self) -> Node {
        self.into()
    }
}

impl Into<Node> for FlexColumn {
    fn into(self) -> Node {
        Node {
            width: self.width,
            height: self.height,
            margin: self.margin,
            justify_content: self.justify,
            align_items: self.align,
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
pub fn node(width: Val, height: Val) -> Node {
    Node {
        width,
        height,
        ..default()
    }
}
