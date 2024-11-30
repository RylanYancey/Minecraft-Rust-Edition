use bevy::{prelude::*, window::WindowResized};

#[derive(Component, Copy, Clone)]
pub struct ScaleableUiWidth(pub f32);

#[derive(Component, Copy, Clone)]
pub struct ScaleableUiHeight(pub f32);

#[derive(Component, Copy, Clone)]
pub struct ScaleableUi(pub f32, pub f32);

#[derive(Resource)]
pub struct UiScale {
    pub factor: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for UiScale {
    fn default() -> Self {
        Self {
            factor: 720.0 / 1440.0,
            width: 1280.0,
            height: 720.0,
        }
    }
}

pub fn update_ui_scale_on_resize(
    mut ui_scale: ResMut<UiScale>,
    mut event: EventReader<WindowResized>,
) {
    for ev in event.read() {
        *ui_scale = UiScale {
            factor: f32::min(ev.width, ev.height) / 1440.0,
            width: ev.width,
            height: ev.height,
        }
    }
}

pub fn update_scaleable_ui(
    ui_scale: Res<UiScale>,
    mut query: Query<(&mut Node, &ScaleableUi), Changed<Node>>,
) {
    for (mut node, scaleable) in &mut query {
        if let Some(width) = val(&mut node.width) {
            *width = scaleable.0 * ui_scale.factor;
        }

        if let Some(height) = val(&mut node.height) {
            *height = scaleable.1 * ui_scale.factor;
        }
    }
}

pub fn update_scaleable_ui_width(
    ui_scale: Res<UiScale>,
    mut query: Query<(&mut Node, &ScaleableUiWidth), Changed<Node>>,
) {
    for (mut style, scaleable) in &mut query {
        if let Some(width) = val(&mut style.width) {
            *width = scaleable.0 * ui_scale.factor;
        }
    }
}

pub fn update_scaleable_ui_height(
    ui_scale: Res<UiScale>,
    mut query: Query<(&mut Node, &ScaleableUiHeight), Changed<Node>>,
) {
    for (mut node, scaleable) in &mut query {
        if let Some(height) = val(&mut node.height) {
            *height = scaleable.0 * ui_scale.factor;
        }
    }
}

fn val(val: &mut Val) -> Option<&mut f32> {
    Some(match val {
        Val::Percent(ref mut v) => v,
        Val::Px(ref mut v) => v,
        Val::Vw(ref mut v) => v,
        Val::Vh(ref mut v) => v,
        Val::VMin(ref mut v) => v,
        Val::VMax(ref mut v) => v,
        Val::Auto => return None,
    })
}
