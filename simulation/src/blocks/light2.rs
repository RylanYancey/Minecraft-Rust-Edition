
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Light(u16);

impl Light {
    pub const ZERO: Self = Self(0);

    /// Construct a new Light from raw values.
    /// - ambient: [0, 15],
    /// - intensity: [0, 15],
    /// - hue: [0, 15],
    /// - lightness: [0, 15]
    pub fn from_raw(ambient: u8, intensity: u8, hue: u8, lightness: u8) -> Self {
        Self(0)
    }
}
