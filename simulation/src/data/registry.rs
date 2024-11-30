use std::{collections::BTreeMap, fmt::Debug, ops::{Deref, DerefMut}};
use bevy::prelude::*;
use super::Id;

#[derive(Resource)]
pub struct Registry<I: 'static> {
    name: String,
    entries: Vec<Entry<I>>,
    map: BTreeMap<GlobalID, u32>,
}

impl<I: 'static> Registry<I> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::with_capacity(1024),
            map: BTreeMap::new()
        }
    }

    pub fn add(&mut self, id: Id, item: I) {
        let index = self.entries.len() as u32;
        self.entries.push(Entry { id, index, item});
        
        if let Some(existing) = self.map.insert(GlobalID(id.id()), index) {
            log::error!("Inserted entry with name '{}' into registry '{}', but an entry with name '{}' has the same hash '{}'!", 
                id.name(), self.name, self.entries[existing as usize].id.name(), id.id());    
        } else {
            log::info!("Inserted entry with name '{}' into registry '{}'", id.name(), self.name);
        }
    }

    pub fn get_by_local(&self, local: LocalID) -> &Entry<I> {
        &self.entries[local.0 as usize]
    }

    pub fn get_by_global(&self, global: GlobalID) -> Option<&Entry<I>> {
        self.map.get(&global).map(|index| &self.entries[*index as usize])
    }
}

pub struct Entry<I> {
    /// The Entries name and hash.
    id: Id,

    /// The Index of the entry in the `entries` vec.
    index: u32,

    /// the Item itself.
    item: I,
}

impl<I> Entry<I> {
    /// Get the ID, which contains both
    /// the name of the entry and its hash.
    pub fn id(&self) -> Id {
        self.id
    }

    /// Get the GlobalID, which contains the
    /// hash of the entry's name from the Id.
    pub fn global_id(&self) -> GlobalID {
        GlobalID(self.id.id())
    }

    /// Get the LocalID, which contains the
    /// index of the entry in the Registry.
    pub fn local_id(&self) -> LocalID {
        LocalID(self.index)
    }
}

impl<I> Deref for Entry<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<I> DerefMut for Entry<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

/// The GlobalID of a Registry Entry
/// is the hash of its name. This hash
/// is consistent across versions and
/// platforms, so long as the name of 
/// the entry stays the same.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalID(u32);

impl GlobalID {
    pub fn new(hash: u32) -> Self {
        Self(hash)
    }
    
    pub fn hash(&self) -> u32 {
        self.0
    }
}

/// The LocalID of a Registry Entry 
/// is the index within the `entries` vec
/// the entry resides. This index is based
/// on insertion order and is guaranteed
/// to NOT be consistent across versions.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalID(u32);

impl LocalID {
    pub fn new(index: u32) -> Self {
        Self(index)
    }
    
    pub fn index(&self) -> u32 {
        self.0
    }
}
