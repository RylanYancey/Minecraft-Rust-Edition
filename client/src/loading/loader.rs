use bevy::{
    ecs::{
        intern::Interned,
        schedule::ScheduleLabel,
    },
    prelude::*,
};

#[derive(Default)]
pub struct Loader {
    /// Vector of schedules in the order they should be ran.
    pub stages: Vec<(Schedule, String)>,
    pub curr: usize,
}

impl Loader {
    pub fn add_stage(&mut self, label: impl ScheduleLabel, hint: impl Into<String>) {
        if self.find_stage(label.intern()).is_some() {
            panic!("Attempted to add load stage with label '{label:?}, but it already exists.")
        }

        self.stages.push((Schedule::new(label), hint.into()));
    }

    pub fn add_systems<M>(
        &mut self,
        label: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) {
        let label = label.intern();
        for (stage, _) in &mut self.stages {
            if stage.label() == label {
                stage.add_systems(systems);
                return;
            }
        }

        panic!(
            "Attempted to add systems into load stage with label '{label:?}' but it did not exist!"
        );
    }

    pub fn is_done(&self) -> bool {
        self.curr == self.stages.len()
    }

    pub fn progress(&self) -> f32 {
        self.curr as f32 / self.stages.len() as f32
    }

    pub fn next_stage(&mut self) -> Option<(&mut Schedule, String)> {
        self.curr += 1;
        let next_hint = self
            .stages
            .get(self.curr)
            .map(|some| some.1.clone())
            .unwrap_or_else(|| "Done!".to_string());

        if let Some((stage, _)) = self.stages.get_mut(self.curr - 1) {
            Some((stage, next_hint))
        } else {
            None
        }
    }

    fn find_stage(&self, label: Interned<dyn ScheduleLabel>) -> Option<&(Schedule, String)> {
        for stage in &self.stages {
            if stage.0.label() == label {
                return Some(stage);
            }
        }

        None
    }
}
