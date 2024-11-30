
use super::{Id, SortedSet};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TagSet(SortedSet<Tag>);

impl TagSet {
    pub fn new() -> Self {
        Self(SortedSet::new())
    }

    pub fn with(mut self, tag: Tag) -> Self {
        self.0.insert(tag);
        self
    }

    pub fn add(&mut self, tag: Tag) {
        self.0.insert(tag);
    }

    pub fn remove(&mut self, tag: &Tag) -> bool {
        self.0.remove(tag)
    }

    pub fn has(&self, tag: &Tag) -> bool {
        self.0.contains(tag)
    }

    pub fn iter(&self) -> impl Iterator<Item=&Tag> {
        self.0.iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct Tag(pub Id);

impl Tag {
    pub const fn new(id: &'static str) -> Self {
        Self(Id::new(id))
    }

    pub const fn id(&self) -> u32 {
        self.0.id()
    }

    pub const fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        Self(Id::from(value))
    }
}

impl From<Id> for Tag {
    fn from(value: Id) -> Self {
        Self(value)
    }
}
