
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SortedMap<K, V>(Vec<(K, V)>);

impl<K, V> SortedMap<K, V> {
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl<K, V> SortedMap<K, V>
where
    K: Ord
{
    pub fn insert(&mut self, key: K, val: V) -> bool {
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(index) => { self.0[index].1 = val; true },
            Err(index) => { self.0.insert(index, (key, val)); false }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(index) => { Some(self.0.remove(index).1) },
            Err(_) => None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(index) => self.0.get(index).map(|(_, v)| v),
            Err(_) => return None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(key)) {
            Ok(index) => self.0.get_mut(index).map(|(_, v)| v),
            Err(_) => return None,
        }
    }

    pub fn keys(&self) -> impl Iterator<Item=&K> {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn iter(&self) -> impl Iterator<Item=&(K, V)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&K, &mut V)> {
        self.0.iter_mut().map(|(k, v)| (&*k, v))
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SortedSet<K>(SortedMap<K, ()>);

impl<K: Ord> SortedSet<K> {
    pub const fn new() -> Self {
        Self(SortedMap::new())
    }

    pub fn insert(&mut self, key: K) -> bool {
        self.0.insert(key, ())
    }

    pub fn remove(&mut self, key: &K) -> bool {
        self.0.remove(key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.0.contains(key)
    }

    pub fn iter(&self) -> impl Iterator<Item=&K> {
        self.0.keys()
    }
}
