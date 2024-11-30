use std::collections::BTreeMap;
use xxhash_rust::const_xxh32::xxh32;
use std::sync::Mutex;
use std::hash::Hash;

/// I don't want to have to store an owned String for every single id, so
/// I'm just going to store them in this static map. 
static ID_STORAGE: Mutex<BTreeMap<u32, String>> = Mutex::new(BTreeMap::new());
const ID_HASH_SEED: u32 = 0xDCA3875F;

#[derive(Copy, Clone, Eq, Ord, Debug)]
pub struct Id {
    name: &'static str,
    id: u32,
}

impl Id {
    pub const fn new(id: &'static str) -> Self {
        Self {
            name: id, 
            id: xxh32(id.as_bytes(), ID_HASH_SEED)
        }
    }

    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[inline]
    pub const fn id(&self) -> u32 {
        self.id
    }
}

impl<T> From<T> for Id 
where
    T: Into<String>
{
    fn from(value: T) -> Self {
        let str = value.into();
        let hash = xxh32(str.as_bytes(), ID_HASH_SEED);

        let id = unsafe {
            std::mem::transmute::<&str, &'static str>(ID_STORAGE.lock().unwrap().entry(hash).or_insert(str).as_str())
        };
        
        Self {
            id: hash, 
            name: id
        }
    }
}

impl std::ops::Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
