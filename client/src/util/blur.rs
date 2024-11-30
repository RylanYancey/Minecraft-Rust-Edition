use bevy::{
    prelude::*,
    render::{
        render_graph::RenderGraph,
        render_resource::{AsBindGroup, ShaderModule, ShaderRef},
    },
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlurMaterial {
    #[uniform(0)]
    pub blur_strength: i32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
    pub alpha_mode: AlphaMode,
}

impl Default for BlurMaterial {
    fn default() -> Self {
        Self {
            blur_strength: 2,
            color_texture: Handle::default(),
            alpha_mode: AlphaMode::default(),
        }
    }
}

impl Material for BlurMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blur.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
