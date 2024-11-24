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

