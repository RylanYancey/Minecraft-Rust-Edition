use std::{fmt::Debug, ops::{Deref, DerefMut}};

use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Registry<I: RegistryItem> {
    entries: Vec<(I::ID, Entry<I>)>,
}

impl<I: RegistryItem> Registry<I> {
    pub fn add(&mut self, item: I, name: &str) {
        let id = I::ID::from(name.to_string());

        match self.entries.binary_search_by(|(other_id, _)| id.cmp(other_id)) {
            Ok(i) => {
                panic!(
                    "Attempted to insert into registry '{}' an item with name '{name}', but an existing entry with name '{}' has the same ID.", 
                    I::registry_name(), self.entries[i].1.name()
                )
            },

            Err(i) => {
                self.entries.insert(i, (
                    id,
                    Entry { 
                        name: name.to_string(), 
                        id, item 
                    }
                ));
            }
        }
    }

    /// Get the entry with the id, panicking if it does not exist.
    pub fn find(&self, key: I::ID) -> Option<&Entry<I>> {
        match self.entries.binary_search_by(|(other_id, _)| key.cmp(other_id)) {
            Ok(i) => Some(&self.entries[i].1,),
            Err(_) => None
        }
    }

    /// Get the entry with the id, panicking if it does not exist.
    pub fn find_mut(&mut self, key: I::ID) -> Option<&mut Entry<I>> {
        match self.entries.binary_search_by(|(other_id, _)| key.cmp(other_id)) {
            Ok(i) => Some(&mut self.entries[i].1),
            Err(_) => None
        }
    }

    /// Find the element with the name, throwing an error if it does not exist.
    pub fn expect(&self, name: &str) -> &Entry<I> {
        self.find(I::ID::from(name.to_string())).unwrap_or_else(|| {
            panic!("Expected an element with name {name} from registry {} to exist, but it did not.", I::registry_name())
        })
    }

    /// Find the element with the name, throwing an error if it does not exist.
    pub fn expect_mut(&mut self, name: &str) -> &mut Entry<I> {
        self.find_mut(I::ID::from(name.to_string())).unwrap_or_else(|| {
            panic!("Expected an element with name {name} from registry {} to exist, but it did not.", I::registry_name())
        })
    }

    pub fn iter(&self) -> impl Iterator<Item=&Entry<I>> {
        self.entries.iter().map(|(_, entry)| entry)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Entry<I>> {
        self.entries.iter_mut().map(|(_, entry)| entry)
    }
}

pub struct Entry<I: RegistryItem> {
    name: String,
    id: I::ID,
    pub item: I,
}

impl<I: RegistryItem> Entry<I> {
    pub fn name(&self) -> &'static str {
        unsafe {
            std::mem::transmute(&*self.name)
        }
    }

    pub fn id(&self) -> &I::ID {
        &self.id
    }
}

impl<I: RegistryItem> Deref for Entry<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<I: RegistryItem> DerefMut for Entry<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

pub trait RegistryItem {
    type ID: PartialOrd + Ord + From<String> + Copy + Debug;

    fn registry_name() -> &'static str;
    fn id(&self) -> Self::ID;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct LocalID(pub u32);

impl LocalID {
    pub fn id(&self) -> u32 {
        self.0
    }
}

impl Deref for LocalID {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}