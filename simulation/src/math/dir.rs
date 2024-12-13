use std::ops::Add;

use bevy::math::IVec3;

use super::Vec3;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dir {
    Up = 0,
    Down = 1,
    East = 2,
    West = 3,
    North = 4,
    South = 5,
}

impl Dir {
    pub fn to_vec3(&self) -> Vec3<i32> {
        match *self {
            Self::Up => Vec3::up(),
            Self::Down => Vec3::down(),
            Self::East => Vec3::east(),
            Self::West => Vec3::west(),
            Self::North => Vec3::north(),
            Self::South => Vec3::south(),
        }
    }

    pub fn from_vec3(vec3: Vec3<i32>) -> Self {
        match vec3 {
            Vec3(0, 1, 0) => Self::Up,
            Vec3(0,-1, 0) => Self::Down,
            Vec3(1, 0, 0) => Self::East,
            Vec3(-1,0, 0) => Self::West,
            Vec3(0, 0, 1) => Self::North,
            Vec3(0, 0,-1) => Self::South,
            _ => panic!("Vec3 is not a valid direction: {vec3:?}")
        }
    }

    pub fn to_index(&self) -> usize {
        (*self as isize) as usize
    }

    pub fn invert(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::North => Self::South,
        }
    }
}

impl Into<Vec3<i32>> for Dir {
    fn into(self) -> Vec3<i32> {
        self.to_vec3()
    }
}

impl From<Vec3<i32>> for Dir {
    fn from(value: Vec3<i32>) -> Self {
        Self::from_vec3(value)
    }
}

impl Add<IVec3> for Dir {
    type Output = IVec3;

    fn add(self, rhs: IVec3) -> Self::Output {
        match self {
            Self::Up => rhs.with_y(rhs.y + 1),
            Self::Down => rhs.with_y(rhs.y - 1),
            Self::East => rhs.with_x(rhs.x + 1),
            Self::West => rhs.with_x(rhs.x - 1),
            Self::North => rhs.with_z(rhs.z + 1),
            Self::South => rhs.with_z(rhs.z - 1)
        }
    }
}
