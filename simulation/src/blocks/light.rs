
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Light {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Light {
    pub const fn default() -> Self {
        Self {
            a: 255,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

