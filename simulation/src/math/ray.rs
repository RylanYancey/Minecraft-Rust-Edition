use super::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub dir: Vec3<f32>,
}

impl Ray {
    pub fn new(origin: Vec3<f32>, dir: Vec3<f32>) -> Self {
        Self { origin, dir }
    }
}