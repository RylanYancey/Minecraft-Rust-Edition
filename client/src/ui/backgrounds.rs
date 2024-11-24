use super::{
    helper::{node, NodeBundleExt},
    menus::{MAX_MENU_CONTENT_WIDTH, MENU_CONTENT_WIDTH},
};
use crate::{
    camera::MainCamera,
    util::{blur::BlurMaterial, spin::Spin},
};
use bevy::{
    prelude::*,
    render::mesh::{RectangleMeshBuilder, SphereKind, SphereMeshBuilder},
};
use std::{
    borrow::BorrowMut,
    f32::consts::{FRAC_PI_2, PI},
};

use super::{helper::FlexColumn, MenuRoot};

#[derive(Resource, Default)]
pub struct Panorama {
    pub images: Vec<Handle<Image>>,
}

#[derive(Component)]
pub struct PanoramaFace;

/// runs on startup to load the Panorama images
pub fn load_panoramic_images(mut panorama: ResMut<Panorama>, assets: Res<AssetServer>) {
    for i in 0..6 {
        panorama
            .images
            .push(assets.load(format!("menus/panorama/panorama_{i}.png")));
    }
}

/// runs when entering The SinglePlayer, Multiplayer,
/// or Title Menu from a None MenuState.
pub fn draw_panoramic_background(
    panorama: Res<Panorama>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<BlurMaterial>>,
    mut query: Query<(&mut Transform, Entity), With<MainCamera>>,
) {
    pub const SKYBOX_SIZE: f32 = 1000.0;

    let skybox = meshes.add(Rectangle::from_size(Vec2::splat(SKYBOX_SIZE)));

    // (direction, angle)
    // these values are laid out in the order that they appear in the 'panorama' folder.
    // 0, 1, 2, 3, 4, 5
    let sides: [(Vec3, Quat); 6] = [
        // front
        (Vec3::NEG_Z, Quat::IDENTITY * Quat::from_rotation_z(PI)),
        // right
        (
            Vec3::X,
            Quat::from_rotation_y(-FRAC_PI_2) * Quat::from_rotation_z(PI),
        ),
        // back
        (
            Vec3::Z,
            Quat::from_rotation_y(PI) * Quat::from_rotation_z(PI),
        ),
        // left
        (
            Vec3::NEG_X,
            Quat::from_rotation_y(FRAC_PI_2) * Quat::from_rotation_z(PI),
        ),
        // above
        (
            Vec3::Y,
            Quat::from_rotation_x(-FRAC_PI_2) * Quat::from_rotation_z(PI),
        ),
        // below
        (
            Vec3::NEG_Y,
            Quat::from_rotation_x(FRAC_PI_2) * Quat::from_rotation_z(PI),
        ),
    ];

    for (i, image) in panorama.images.iter().enumerate() {
        let material = materials.add(BlurMaterial {
            blur_strength: 1,
            color_texture: image.clone(),
            ..default()
        });

        let (direction, angle) = sides[i];

        commands.spawn((
            PanoramaFace,
            MaterialMeshBundle {
                mesh: skybox.clone(),
                material,
                transform: Transform {
                    translation: direction * (SKYBOX_SIZE / 2.0),
                    rotation: angle,
                    scale: Vec3::splat(-1.0),
                    ..default()
                },
                ..default()
            },
        ));
    }

    for (mut transform, entity) in &mut query {
        commands.entity(entity).insert(Spin(-0.05));
        *transform = Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
}

pub const BLUR_LOW: i32 = 0;
pub const BLUR_HIGH: i32 = 6;

/// Set the panorama blur to N
pub fn set_panorama_blur<const N: i32>(
    mut materials: ResMut<Assets<BlurMaterial>>,
    mut query: Query<&mut Handle<BlurMaterial>, With<PanoramaFace>>,
) {
    for mut material in &mut query {
        if let Some(mat) = materials.get(material.id()) {
            let color_texture = mat.color_texture.clone();
            let alpha_mode = mat.alpha_mode.clone();

            *material = materials.add(BlurMaterial {
                blur_strength: N,
                color_texture,
                alpha_mode,
            })
        }
    }
}

// Background used on selector menus like WorldSelect,
// ServerSelect, and the Options Menu.
pub fn spawn_select_menu_root<FTop, FCenter, FBottom>(
    commands: &mut Commands,
    top: FTop,
    center: FCenter,
    bottom: FBottom,
) where
    FTop: Fn(&mut ChildBuilder),
    FCenter: Fn(&mut ChildBuilder),
    FBottom: Fn(&mut ChildBuilder),
{
    commands
        .spawn((
            MenuRoot,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()  
                },
                ..default()
            },
        ))
        .with_children(|root| {
            // top of menu
            root.spawn(node(Val::Percent(100.0), Val::Percent(15.0)))
                .with_children(|parent| {
                    parent
                        .spawn(
                            Style {
                                width: MENU_CONTENT_WIDTH,
                                max_width: MAX_MENU_CONTENT_WIDTH,
                                height: Val::Percent(100.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::horizontal(Val::Auto),
                                ..default()
                            }
                            .node(),
                        )
                        .with_children(top);
                });

            // central panel, with darkened content and border.
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(65.0),
                    border: UiRect::axes(Val::Px(0.0), Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                border_color: BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(
                        Style {
                            width: MENU_CONTENT_WIDTH,
                            max_width: MAX_MENU_CONTENT_WIDTH,
                            height: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::horizontal(Val::Auto),
                            ..default()
                        }
                        .node(),
                    )
                    .with_children(center);
            });

            // bottom panel
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(
                        Style {
                            width: MENU_CONTENT_WIDTH,
                            max_width: MAX_MENU_CONTENT_WIDTH,
                            height: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect::horizontal(Val::Auto),
                            row_gap: Val::Px(10.0),
                            ..default()
                        }
                        .node(),
                    )
                    .with_children(bottom);
            });
        });
}
