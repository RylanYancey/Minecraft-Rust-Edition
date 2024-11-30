use super::{ray::Ray, Vec3};

/// A Very Simple Axis Aligned Bounding Box.
/// This struct enforces the bounding box
/// to be aligned to the axes.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BoundingBox {
    pub origin: Vec3<f32>,
    pub extent: Vec3<f32>,
}

impl BoundingBox {
    pub fn new(origin: Vec3<f32>, extent: Vec3<f32>) -> Self {
        Self{origin, extent}
    }

    pub fn block() -> Self {
        Self {
            origin: Vec3::splat(0.0),
            extent: Vec3::splat(1.0),
        }
    }

    /// BoundingBoxes for blocks are provided relative
    /// to block origin. Use this function to offset it
    /// to world-space origin. Provided 'offs' should be
    /// the world-space block pos in that case.
    pub fn offset(&self, offs: Vec3<f32>) -> Self {
        Self {
            origin: self.origin + offs,
            extent: self.extent,
        }
    }

    /// Detect if two Bounding Boxes are overlapping.
    pub fn overlaps(&self, other: &Self) -> bool {
        let Vec3(x1, y1, z1) = self.origin;
        let Vec3(x2, y2, z2) = other.origin;
        let Vec3(w1, h1, d1) = self.extent;
        let Vec3(w2, h2, d2) = other.extent;

        x1 < x2 + w2 && x1 + w1 > x2 &&
        y1 < y2 + h2 && y1 + h1 > y2 &&
        z1 < z2 + d2 && z1 + d1 > z2
    }

    /// Whether the ray intersects this block.
    pub fn intersects_ray(&self, ray: &Ray) -> Option<Vec3<f32>> {
        let Vec3(ox, oy, oz) = ray.origin;
        let Vec3(dx, dy, dz) = ray.dir;
        let Vec3(bx_min, by_min, bz_min) = self.origin;
        let Vec3(bx_max, by_max, bz_max) = self.origin + self.extent;

        let (mut t_min, mut t_max) = (f32::NEG_INFINITY, f32::INFINITY);

        // Helper function to calculate t1 and t2 for each axis
        let check_axis = |origin: f32, direction: f32, b_min: f32, b_max: f32| -> (f32, f32) {
            if direction != 0.0 {
                let t1 = (b_min - origin) / direction;
                let t2 = (b_max - origin) / direction;
                if t1 < t2 {
                    (t1, t2)
                } else {
                    (t2, t1)
                }
            } else {
                if origin < b_min || origin > b_max {
                    (f32::INFINITY, f32::NEG_INFINITY)
                } else {
                    (f32::NEG_INFINITY, f32::INFINITY)
                }
            }
        };
    
        // Check intersection on the x axis
        let (t1, t2) = check_axis(ox, dx, bx_min, bx_max);
        t_min = t_min.max(t1);
        t_max = t_max.min(t2);
    
        // Check intersection on the y axis
        let (t1, t2) = check_axis(oy, dy, by_min, by_max);
        t_min = t_min.max(t1);
        t_max = t_max.min(t2);
    
        // Check intersection on the z axis
        let (t1, t2) = check_axis(oz, dz, bz_min, bz_max);
        t_min = t_min.max(t1);
        t_max = t_max.min(t2);
    
        // If t_min <= t_max, we have an intersection
        if t_min <= t_max {
            // Compute the intersection point at t_min
            let ix = ox + t_min * dx;
            let iy = oy + t_min * dy;
            let iz = oz + t_min * dz;
            Some(Vec3(ix, iy, iz))
        } else {
            None
        }
    }

    /// Get the normal (direction) of the face nearest to the point.
    /// Used for computing where to place blocks when you try to place
    /// on a partial block like a slab or torch.
    pub fn nearest_face_normal(&self, point: Vec3<f32>) -> Vec3<f32> {
        let Vec3(px, py, pz) = point;
        let Vec3(bx_min, by_min, bz_min) = self.origin;
        let Vec3(bx_max, by_max, bz_max) = self.origin + self.extent;

        // Compute distances to each face
        let dist_left = (px - bx_min).abs();
        let dist_right = (px - bx_max).abs();
        let dist_bottom = (py - by_min).abs();
        let dist_top = (py - by_max).abs();
        let dist_front = (pz - bz_min).abs();
        let dist_back = (pz - bz_max).abs();

        // Determine which distance is the smallest
        let min_dist = dist_left
            .min(dist_right)
            .min(dist_bottom)
            .min(dist_top)
            .min(dist_front)
            .min(dist_back);

        // Return the normal of the closest face
        if min_dist == dist_left {
            Vec3(-1.0, 0.0, 0.0) // Left face
        } else if min_dist == dist_right {
            Vec3(1.0, 0.0, 0.0) // Right face
        } else if min_dist == dist_bottom {
            Vec3(0.0, -1.0, 0.0) // Bottom face
        } else if min_dist == dist_top {
            Vec3(0.0, 1.0, 0.0) // Top face
        } else if min_dist == dist_front {
            Vec3(0.0, 0.0, -1.0) // Front face
        } else {
            Vec3(0.0, 0.0, 1.0) // Back face
        }
    }
}
