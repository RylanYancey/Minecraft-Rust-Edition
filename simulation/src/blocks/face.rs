use crate::math::Dir;

/// A Description of the faces of a block.
/// The data is stored as (Face, bool), where
/// the Face is the coverage of the block and
/// the boolean indicates whether or not the
/// block is transparent.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Faces {
    /// Descriptions of the faces of a block.
    /// In the order Up, Down, East, West, North, South.
    pub faces: [Face; 6],

    /// A bitmask of face transparency used for
    /// optimizing light computation. Each '1'
    /// in this mask corresponds to a direction
    /// of a face on the block that allows light
    /// to pass into the block.
    pub bitmask: Transparency,
}

impl Faces {
    pub fn all(face: Face) -> Self {
        Self {
            faces: [face; 6],
            bitmask: Transparency(if face.is_transparent() {
                0b00111111
            } else {
                0b0
            }),
        }
    }

    pub fn with(mut self, dir: Dir, face: Face) -> Self {
        self.faces[dir.to_index()] = face;
        if face.is_transparent() {
            self.bitmask |= dir.into();
        }
        self
    }

    pub fn set(&mut self, dir: Dir, face: Face) {
        self.faces[dir.to_index()] = face;
        if face.is_transparent() {
            self.bitmask |= dir.into();
        }
    }

    /// Get the face in the specified direction.
    /// The boolean returned indicates whether or
    /// not the face is transparent.
    pub fn get(&self, dir: Dir) -> Face {
        self.faces[dir.to_index()]
    }

    pub fn up(&self) -> Face {
        self.faces[Dir::Up.to_index()]
    }

    pub fn down(&self) -> Face {
        self.faces[Dir::Down.to_index()]
    }

    pub fn east(&self) -> Face {
        self.faces[Dir::East.to_index()]
    }

    pub fn west(&self) -> Face {
        self.faces[Dir::West.to_index()]
    }

    pub fn north(&self) -> Face {
        self.faces[Dir::North.to_index()]
    }

    pub fn south(&self) -> Face {
        self.faces[Dir::South.to_index()]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Face {
    pub transparent: bool,
    pub coverage: FaceCoverage,
}

impl Face {
    /// Returns `true` if the face is transparent or
    pub fn is_transparent(&self) -> bool {
        self.coverage != FaceCoverage::Full || self.transparent
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FaceCoverage {
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
        width: Pixels,
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
            _ => panic!("Invalid Face Width: {u}"),
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

/// Whether or not the faces of a block allow
/// light to pass through them.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Transparency(pub u8);

impl Transparency {
    /// Returns true if self has all the same
    /// transparent faces as rhs.
    pub fn has(&self, rhs: Self) -> bool {
        *self & rhs == rhs
    }
}

impl From<Dir> for Transparency {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Self::UP,
            Dir::Down => Self::DOWN,
            Dir::East => Self::EAST,
            Dir::West => Self::WEST,
            Dir::North => Self::NORTH,
            Dir::South => Self::SOUTH,
        }
    }
}

bitflags::bitflags! {
    impl Transparency: u8 {
        const UP    = 0b0000_0001;
        const DOWN  = 0b0000_0010;
        const EAST  = 0b0000_0100;
        const WEST  = 0b0000_1000;
        const NORTH = 0b0001_0000;
        const SOUTH = 0b0010_0000;
    }
}
