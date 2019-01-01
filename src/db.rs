use crate::data::{Id, Item};
use std::collections::HashMap;
use std::hash::Hash;

pub trait Database<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self;
    fn init() -> Self;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn get_all(&self) -> &HashMap<K, V>;
}

#[derive(Debug)]
pub struct HashMapDb<K, V>
where
    K: Hash + Eq,
{
    store: HashMap<K, V>,
}

impl<K, V> Database<K, V> for HashMapDb<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        HashMapDb {
            store: HashMap::new(),
        }
    }
    fn init() -> Self {
        Self::new()
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.store.insert(key, value)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.store.remove(key)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.store.get_mut(key)
    }
    fn get_all(&self) -> &HashMap<K, V> {
        &self.store
    }
}

pub trait PersistentDatabase {
    fn load(&mut self);
    fn save(&self);
}

impl PersistentDatabase for HashMapDb<Id, Item> {
    fn load(&mut self) {
        // for note_text in vec!["One", "Two", "Three"] {
        //     let mut note = Skribe::create_item();
        //     note.text = String::from(note_text);
        //     self.insert(note.id, note);
        // }
    }
    fn save(&self) {}
}
