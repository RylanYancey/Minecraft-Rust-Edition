use crate::math::Dir;

/// A Description of the faces of a block.
/// The data is stored as (Face, bool), where
/// the Face is the coverage of the block and
/// the boolean indicates whether or not the
/// block is transparent. 
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Faces([(Face, bool); 6]);

impl Faces {
    pub fn all(face: Face, transparent: bool) -> Self {
        Self([(face, transparent); 6])
    }

    pub fn full() -> Self {
        Self([(Face::Full, false); 6])
    }

    pub fn air() -> Self {
        Self::default()
    }

    pub fn with(mut self, dir: Dir, face: Face, transparency: bool) -> Self {
        self.0[dir.to_index()] = (face, transparency);
        self
    }

    /// Get the face in the specified direction.
    /// The boolean returned indicates whether or
    /// not the face is transparent.
    pub fn get(&self, dir: Dir) -> (Face, bool) {
        self.0[dir.to_index()]
    }

    pub fn up(&self) -> (Face, bool) {
        self.0[Dir::Up.to_index()]
    }

    pub fn down(&self) -> (Face, bool) {
        self.0[Dir::Down.to_index()]
    }

    pub fn east(&self) -> (Face, bool) {
        self.0[Dir::East.to_index()]
    }

    pub fn west(&self) -> (Face, bool) {
        self.0[Dir::West.to_index()]
    }

    pub fn north(&self) -> (Face, bool) {
        self.0[Dir::North.to_index()]
    }

    pub fn south(&self) -> (Face, bool) {
        self.0[Dir::South.to_index()]
    }
}

impl From<(Face, bool)> for Faces {
    fn from(value: (Face, bool)) -> Self {
        Self([value; 6])
    }
}

impl Default for Faces {
    fn default() -> Self {
        Self([(Face::None, false); 6])
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Face {
    //// The face exists and takes up
    /// 100% of the face space.
    /// 
    /// Examples: Glass Block, Stone Block.
    Full,

    /// The face does not exist.
    /// 
    /// Examples: Air, Grass
    None,

    /// The face exists, but only a part of the block.
    /// The Direction indicates the side of the block 
    /// that is up against the edge and extents from corner
    /// to corner.
    /// 
    /// For example, for the south side of a bottom slab,
    /// this would be Face::Half(Dir::Down, Pixels::Px8).
    /// 
    /// Examples: Half Slab
    Half {
        side: Dir, 
        width: Pixels
    },

    /// The face is inset into the block.
    /// Used for in-set torches and other
    /// decorative blocks.
    /// 
    /// Examples: 
    ///  - Farmland: Face::Inset { torchable: None, distance: Pixels::Px1 }
    ///  - Wall: Face::Inset { torchable: Some(Pixels::Px7), distance: Pixels::Px6 }
    ///  - Top of Slab: Face::Inset { torchable: Some(Pixels::Px8), distance: Pixels::Px8 }
    ///  - Front of Pane: Face::Inset { torchable: None, distance: Pixels::Px7 }
    /// 
    /// The torchable parameter refers to whether or not a torch (or similar decorative block)
    /// can be placed onthe inset face. If so, the pixel value refers to the center where the
    /// block should sit within the face. So, if you want to shift the inset block down 1
    /// when placed against this block, you would do torchable: Some(Pixels::Px7)
    Inset {
        torchable: Option<Pixels>,
        distance: Pixels,
    }, 

    Cross(Pixels),

    /// An L-Shape has two sides with a width of 16
    /// that are along the edge of the block.
    /// 
    /// Examples
    ///  - Stairs
    LShape {
        /// Deg0 = Bottom Left Corner, clockwise.
        rot: FaceRotation,
        width: Pixels,
    },

    /// A Corner is, eh, in the corner.
    Corner {
        /// Deg0 = Bottom Left Corner, clockwise.
        rot: FaceRotation,
        width: Pixels,
    },

    /// The Face is Square and Centered
    /// Examples
    ///  - Bottom of wall: Square(Pixels::Px4)
    Square(Pixels),

    /// The face is pinched on two sides, but 
    /// still has a height of 16. The Direction
    /// must be positive and denotes the side that
    /// has a width of 16.
    /// 
    /// Examples
    ///  - Side of Glass Pane: Pinched { width: Pixels::Px2, dir: Dir::Up }
    Pinched {
        width: Pixels,
        dir: Dir,
    },

    /// The face is 'special'.
    /// Examples:
    ///  - Side of Fence
    Other,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pixels {
    Px1 = 1,
    Px2 = 2,
    Px3 = 3,
    Px4 = 4,
    Px5 = 5,
    Px6 = 6,
    Px7 = 7,
    Px8 = 8,
    Px9 = 9,
    Px10 = 10,
    Px11 = 11,
    Px12 = 12,
    Px13 = 13,
    Px14 = 14,
    Px15 = 15,
}

impl Pixels {
    pub fn to_u8(self) -> u8 {
        (self as i8) as u8
    }

    pub fn from_u8(u: u8) -> Self {
        match u {
            1 => Self::Px1,
            2 => Self::Px2,
            3 => Self::Px3,
            4 => Self::Px4,
            5 => Self::Px5,
            6 => Self::Px6,
            7 => Self::Px7,
            8 => Self::Px8,
            9 => Self::Px9,
            10 => Self::Px10,
            11 => Self::Px11,
            12 => Self::Px12,
            13 => Self::Px13,
            14 => Self::Px14,
            15 => Self::Px15,
            _ => panic!("Invalid Face Width: {u}")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FaceRotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}