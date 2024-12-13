use crate::data::registry::LocalID;
use super::Light;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BlockState {
    pub block: LocalID,
    pub light: Light, 
}

impl Default for BlockState {
    fn default() -> Self {
        Self {
            block: LocalID::new(0),
            light: Light::default()
        }
    }
}
