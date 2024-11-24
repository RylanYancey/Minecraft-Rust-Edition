
use xxhash_rust::xxh32::xxh32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct BlockID(pub u32);

impl BlockID {
    pub const AIR: Self = BlockID(0);

    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> &u32 {
        &self.0
    }
}

impl From<u32> for BlockID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<&str> for BlockID {
    fn from(value: &str) -> Self {
        Self(xxh32(value.as_bytes(), 4206942069))
    }
}