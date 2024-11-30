use bevy::{
    prelude::*,
    render::{render_resource::PipelineCache, MainWorld},
};

#[derive(Resource, Default, Debug)]
pub struct PipelinesReady(pub bool);

impl PipelinesReady {
    pub fn is_ready(&self) -> bool {
        self.0
    }
}

pub fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
        pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0
    }
}
