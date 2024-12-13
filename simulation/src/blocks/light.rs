
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Light(u16);

impl Light {
    pub const ZERO: Self = Self(0);

    pub const fn default() -> Self {
        Self(0xF000)
    }

    /// Construct a new Light from raw values.
    /// - ambient: [0, 15], ambient light level.
    /// - intensity: [0, 15], torch light level.
    /// - hue: [0, 15], HSL Hue.
    /// - lightness: [0, 15], HSL Lightness.
    pub const fn from_raw(ambient: u8, intensity: u8, hue: u8, lightness: u8) -> Self {
        assert!(ambient <= 15, "Ambient must be in the range [0,15]!");
        assert!(intensity <= 15, "Intensity must be in the range [0,15]!");
        assert!(hue <= 15, "Hue must be in the range [0,15]!");
        assert!(lightness <= 15, "Lightness must be in the range [0,15]!");

        Self(
            (ambient as u16)
            | ((intensity as u16) << 4)
            | ((hue as u16) << 8)
            | ((lightness as u16) << 12)
        )
    }

    /// Get the ambient light level.
    pub fn ambient(&self) -> u8 {
        (self.0 & 0xF) as u8
    }

    /// Set the ambient light level.
    pub fn set_ambient(&mut self, ambient: u8) {
        assert!(ambient <= 15, "Ambient must be in range [0, 15]");
        self.0 = (self.0 & !0xF) | (ambient as u16);
    }

    /// Get the intensity level.
    pub fn intensity(&self) -> u8 {
        ((self.0 >> 4) & 0xF) as u8
    }

    /// Set the intensity level.
    pub fn set_intensity(&mut self, intensity: u8) {
        assert!(intensity <= 15, "Intensity must be in range [0, 15]");
        self.0 = (self.0 & !(0xF << 4)) | ((intensity as u16) << 4);
    }

    /// Get the HSL hue.
    pub fn hue(&self) -> u8 {
        ((self.0 >> 8) & 0xF) as u8
    }

    /// Set the HSL hue.
    pub fn set_hue(&mut self, hue: u8) {
        assert!(hue <= 15, "Hue must be in range [0, 15]");
        self.0 = (self.0 & !(0xF << 8)) | ((hue as u16) << 8);
    }

    /// Get the HSL lightness.
    pub fn lightness(&self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }

    /// Set the HSL lightness.
    pub fn set_lightness(&mut self, lightness: u8) {
        assert!(lightness <= 15, "Lightness must be in range [0, 15]");
        self.0 = (self.0 & !(0xF << 12)) | ((lightness as u16) << 12);
    }
}
