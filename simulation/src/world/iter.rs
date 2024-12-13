
use crate::math::Vec3;

pub struct SpatialIter3D {
    pub origin: Vec3<i32>,
    pub extent: Vec3<i32>,
    pub curr: Vec3<i32>,
    pub index: usize,
    pub len: usize,
}

impl SpatialIter3D {
    pub fn new(origin: Vec3<i32>, extent: Vec3<i32>) -> Self {
        Self {
            origin,
            extent,
            curr: Vec3(0, 0, 0),
            index: 0,
            len: extent.prod() as usize
        }
    }
}

impl Iterator for SpatialIter3D {
    type Item = (Vec3<i32>, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            None
        } else {
            let result = Some((self.origin + self.curr, self.index));

            if self.curr.1 == self.extent.1 {
                self.curr.1 = 0;
                self.curr.0 += 1;
                
                if self.curr.0 == self.extent.0 {
                    self.curr.0 = 0;
                    self.curr.2 += 1;    
                }
            }

            self.curr.1 += 1;
            result
        }
    }
}






