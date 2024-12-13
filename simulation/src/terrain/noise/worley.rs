
use crate::math::{Vec2, Vec3};
use crate::terrain::{hash::Permutation, noisemap::{Fbm, Generator}};

pub struct Worley2x3;

impl Generator for Worley2x3 {
    type Output = [f32; 3];
    
    fn sample(&self, pos: Vec3<i32>, fbm: &Fbm, perm: &Permutation) -> [f32; 3] {
        worley2x3(pos.xz().map(|n| *n as f32 * fbm.frequency), perm)
    }
}

/// Compute the distance to the nearest point in a worley grid.
pub fn worley2(pos: Vec2<f32>, perm: &Permutation) -> f32 {
    let mut min = f32::MAX;
    let floor = pos.map(|v| *v as i32);
    for x in -1..1 {
        for z in -1..1 {
            let dist = Vec2(
                perm.x[((floor.x() + x) & 255) as usize] as f32 / 255.0,
                perm.z[((floor.z() + z) & 255) as usize] as f32 / 255.0
            ).distance(pos);

            if dist < min {
                min = dist;
            }
        }
    }

    min
}

pub fn worley2x2(pos: Vec2<f32>, perm: &Permutation) -> [f32; 2] {
    let [a, b, _] = worley2x3(pos, perm);
    [a, b]
}

/// Compute the distances to the 3 nearest points in the worley grid.
pub fn worley2x3(pos: Vec2<f32>, perm: &Permutation) -> [f32; 3] {
    let mut distances = [0.0; 9];
    let mut i = 0;
    let floor = pos.map(|v| *v as i32);
    for x in -1..1 {
        for z in -1..1 {
            let dist = Vec2(
                perm.x[((floor.x() + x) & 255) as usize] as f32 / 255.0,
                perm.z[((floor.z() + z) & 255) as usize] as f32 / 255.0
            ).distance(pos);

            distances[i] = dist;
            i += 1;
        }
    }

    distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    [distances[0], distances[1], distances[2]]
}

/// Compute the distance to the neraest point in a worley grid.
pub fn worley3(pos: Vec3<f32>, perm: &Permutation) -> f32 {
    let mut min = f32::MAX;
    let floor = pos.map(|v| *v as i32);
    for x in -1..1 {
        for y in -1..1 {
            for z in -1..1 {
                let dist = Vec3(
                    perm.x[((floor.x() + x) & 255) as usize] as f32 / 255.0,
                    perm.y[((floor.y() + y) & 255) as usize] as f32 / 255.0,
                    perm.z[((floor.z() + z) & 255) as usize] as f32 / 255.0
                ).distance(pos);

                if dist < min {
                    min = dist;
                }
            }
        }
    }

    min
}

/// Compute the distance to the nearest 3 points in a worley grid.
pub fn worley3x3(pos: Vec3<f32>, perm: &Permutation) -> [f32; 3] {
    let mut min = [f32::MAX; 3];
    let floor = pos.map(|v| *v as i32);
    for x in -1..1 {
        for y in -1..1 {
            for z in -1..1 {
                let dist = Vec3(
                    perm.x[((floor.x() + x) & 255) as usize] as f32 / 255.0,
                    perm.y[((floor.y() + y) & 255) as usize] as f32 / 255.0,
                    perm.z[((floor.x() + z) & 255) as usize] as f32 / 255.0
                ).distance(pos);

                if dist < min[0] {
                    min[2] = min[1];
                    min[1] = min[2];
                    min[0] = dist;
                } else {
                    if dist < min[1] {
                        min[2] = min[1];
                        min[1] = dist;
                    } else {
                        if dist < min[2] {
                            min[2] = dist;
                        }
                    }
                }
            }
        }
    }

    min
}

