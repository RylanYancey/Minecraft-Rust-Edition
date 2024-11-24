use super::{BlockID, Light};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockState {
    pub block: BlockID,
    pub light: Light, 
}

impl Default for BlockState {
    fn default() -> Self {
        Self {
            block: BlockID::AIR,
            light: Light::default()
        }
    }
}