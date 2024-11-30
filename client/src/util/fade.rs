use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_easings::Lerp;

#[derive(Component)]
pub struct FadeOut<C: Fade> {
    pub speed: f32,
    marker: PhantomData<C>,
}

impl<C: Fade> FadeOut<C> {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            marker: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct FadeIn<C: Fade> {
    pub speed: f32,
    marker: PhantomData<C>,
}

impl<C: Fade> FadeIn<C> {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            marker: PhantomData,
        }
    }
}

pub trait Fade: Component {
    fn fade_in(&mut self, speed: f32);
    fn fade_out(&mut self, speed: f32);
}

pub fn fade_in<C: Fade>(mut query: Query<(&FadeIn<C>, &mut C)>) {
    for (fade_in, mut target) in &mut query {
        target.fade_in(fade_in.speed);
    }
}

pub fn fade_out<C: Fade>(mut query: Query<(&FadeOut<C>, &mut C)>) {
    for (fade_out, mut target) in &mut query {
        target.fade_out(fade_out.speed);
    }
}

impl Fade for BackgroundColor {
    fn fade_in(&mut self, speed: f32) {
        self.0.set_alpha(self.0.alpha().lerp(1.0, speed))
    }

    fn fade_out(&mut self, speed: f32) {
        self.0.set_alpha(self.0.alpha().lerp(0.0, speed))
    }
}
