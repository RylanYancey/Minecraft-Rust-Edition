use std::{ops::{Index, IndexMut, Sub, SubAssign}, simd::{cmp::{SimdOrd, SimdPartialEq}, num::SimdUint, Simd}};
use bevy::prelude::Color;

/// Struct for storing Light data.
/// 
/// Lights have 4 Channels:
///  - Ambient: NOT an alpha channel - this is the ambient light.
///  - Red: Colored Red Light at the block
///  - Green: Colored green Light at the block.
///  - Blue: Colored blue Light at the block.
///
/// Channels are in the range [0, 255], _despite_ there 
/// only being 16 possible light states. This is to allow
/// a colored light to stay colored even at the fringes.
/// If you wanted a light to be orange, you would initialize
/// the light like so:
///
/// - orange: Light::new(0, 255, 247, 0)
///
/// The green channel is normally 128 for rgb orange,
/// but here we do 255 - (16 / 2).
///
/// This allows us to have 4096 different colors
/// and 16 different intensities.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Light(Simd<u8, 4>);

impl Light {
    #[inline]
    pub const fn default() -> Self {
        Self(Simd::from_array([0, 0, 0, 0]))
    }

    #[inline]
    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(Simd::from_array([a, r, g, b]))
    }

    /// Construct a new Light from channels (in the range [0,16]).
    /// The individual channels are: (intensity * 16) + channel.
    /// Set ambient light to 0.
    #[inline]
    pub const fn from_color_intensity(intensity: u8, r: u8, g: u8, b: u8) -> Self {
        let intensity = 16 * intensity;
        Self::new(0, r + intensity, g + intensity, b + intensity)
    }

    /// For each channel, return the larger.
    pub fn max(&self, other: &Self) -> Self {
        Self(self.0.simd_max(other.0))
    }

    /// For each channel, return the smaller.
    pub fn min(&self, other: &Self) -> Self {
        Self(self.0.simd_min(other.0))
    }

    /// Compute the value of the maximum channel
    pub fn max_channel(&self) -> u8 {
        self.0.reduce_max()
    }

    /// Compute the value of the lowest channel.
    pub fn min_channel(&self) -> u8 {
        self.0.reduce_min()
    }

    /// Reduce the intensity of this light by 1 step.
    /// Saturating Sub all channels by 16.
    pub fn step_down(&self) -> Self {
        Self(self.0.saturating_sub(Simd::splat(16)))
    }

    pub fn as_array(&self) -> &[u8; 4] {
        self.0.as_array()
    }

    pub fn as_mut_array(&mut self) -> &mut [u8; 4] {
        self.0.as_mut_array()
    }

    pub fn as_simd(&self) -> &Simd<u8, 4> {
        &self.0
    }

    pub fn as_mut_simd(&mut self) -> &mut Simd<u8, 4> {
        &mut self.0
    }

    /// Get the ambient channel.
    pub fn a(&self) -> u8 {self[0]}
    pub fn r(&self) -> u8 {self[1]}
    pub fn g(&self) -> u8 {self[2]}
    pub fn b(&self) -> u8 {self[3]}

    /// Set the ambient channel
    pub fn set_a(&mut self, a: u8) {self[0] = a}
    pub fn set_r(&mut self, r: u8) {self[1] = r}
    pub fn set_g(&mut self, g: u8) {self[2] = g}
    pub fn set_b(&mut self, b: u8) {self[3] = b}

    pub fn with(mut self, channel: usize, value: u8) -> Self {
        self[channel] = value;
        self
    }
    
    pub fn with_a(self, a: u8) -> Self {self.with(0, a)}
    pub fn with_r(self, r: u8) -> Self {self.with(1, r)}
    pub fn with_g(self, g: u8) -> Self {self.with(2, g)}
    pub fn with_b(self, b: u8) -> Self {self.with(3, b)}

    pub const AMBIENT: Self = Self::new(255, 0, 0, 0);
    pub const WHITE: Self = Self::new(0, 255, 255, 255);
    pub const RED: Self = Self::new(0, 255, 0, 0);
    pub const GREEN: Self = Self::new(0, 0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 0, 255);
    pub const YELLOW: Self = Self::new(0, 255, 255, 0);
    pub const CYAN: Self = Self::new(0, 0, 255, 255);
    pub const MAGENTA: Self = Self::new(0, 255, 0, 255);
    pub const ORANGE: Self = Self::new(0, 255, 247, 0);
    pub const TURQUOISE: Self = Self::new(0, 0, 255, 247);
    pub const VIOLET: Self = Self::new(0, 247, 0, 255);
}

impl Index<usize> for Light {
    type Output = u8;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.0.as_array()[index]
    }
}

impl IndexMut<usize> for Light {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0.as_mut_array()[index]
    }
}
