
use xxhash_rust::const_xxh32::xxh32;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TagSet(Vec<Tag>);

impl TagSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Returns this tag set with the provided tag.
    pub fn with(mut self, tag: Tag) -> Self {
        if let Err(i) = self.0.binary_search_by(|other| tag.id.cmp(&other.id)) {
            self.0.insert(i, tag);
        }

        self
    }

    /// Sets the provided tag into the tag set.
    pub fn set(&mut self, tag: Tag) {
        if let Err(i) = self.0.binary_search_by(|other| tag.id.cmp(&other.id)) {
            self.0.insert(i, tag);
        }
    }
    
    /// Returns 'true' if this set has the provided tag.
    pub fn has(&self, tag: &Tag) -> bool {
        self.0.binary_search_by(|other| tag.id.cmp(&other.id)).is_ok()
    }

    /// Returns 'true' if an item was successfully removed from this set.
    pub fn remove(&mut self, tag: &Tag) -> bool {
        if let Ok(i) = self.0.binary_search_by(|other| tag.id.cmp(&other.id)) {
            self.0.remove(i);
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Tag {
    pub name: &'static str,
    pub id: u32,
}

impl Tag {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name, id: xxh32(name.as_bytes(), 4206942069)
        }
    }
}

