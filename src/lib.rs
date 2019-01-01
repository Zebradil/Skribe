#![feature(uniform_paths)]

mod cfg;
mod data;
mod db;

use cfg::Config;
pub use data::{Id, Item};
use db::{Database, HashMapDb, PersistentDatabase};
use failure::Error;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

const CONFIG_PATH: &str = ".skribe";
const SKRIBE_HOME: &str = "SKRIBE_HOME";

#[derive(Debug)]
pub struct Skribe {
    items: HashMapDb<Id, Item>,
    config: Config,
}

impl Skribe {
    pub fn new() -> Skribe {
        let mut items = HashMapDb::new();
        items.load();
        Skribe {
            items: items,
            config: Config::default(),
        }
    }

    pub fn get(&self, id: &Id) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn get_mut(&mut self, id: &Id) -> Option<&mut Item> {
        self.items.get_mut(id)
    }

    pub fn get_items(&self) -> &HashMap<Id, Item> {
        &self.items.get_all()
    }

    // TODO decide to use instance method instead
    pub fn create_item() -> Item {
        Item::default()
    }

    pub fn save_item(&mut self, item: Item) -> Option<Item> {
        self.items.insert(item.id, item)
    }

    pub fn delete_item(&mut self, id: &Id) -> Option<Item> {
        self.items.remove(id)
    }

    pub fn init() -> Result<Self, Error> {
        let config = Config::init(Self::get_config_path())?;
        let items: HashMapDb<Id, Item> = HashMapDb::init();
        Ok(Self { config, items })
    }

    fn get_config_path() -> PathBuf {
        match env::var(SKRIBE_HOME) {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                let home = dirs::home_dir().unwrap_or(PathBuf::from("."));
                home.join(Path::new(CONFIG_PATH))
            }
        }
    }

    pub fn get_home(&self) -> &PathBuf {
        &self.config.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_item() {
        let mut skribe = Skribe::new();
        let item = Skribe::create_item();
        let base_id = item.id;
        let cloned = item.clone();

        // Create
        {
            assert!(
                skribe.get(&base_id).is_none(),
                "There is nothing in the storage at the beginning"
            );

            skribe.save_item(item);
            assert_eq!(
                skribe.get(&base_id).unwrap(),
                &cloned,
                "The new item is saved successfully"
            );
        }

        // Read
        {
            // At the moment there must be only one item
            for (id, item) in skribe.get_items() {
                assert_eq!(&base_id, id);
                assert_eq!(&cloned, item);
            }
        }

        // Update
        {
            {
                let mut editable = skribe.get_mut(&base_id).unwrap();
                editable.title = String::from("Edited");
            }

            assert_eq!(
                skribe.get(&base_id).unwrap().title,
                String::from("Edited"),
                "The item was changed successfully"
            );
        }

        // Delete
        {
            let another_cloned = skribe.get(&base_id).unwrap().clone();
            let deleted = skribe.delete_item(&base_id).unwrap();
            assert_eq!(another_cloned, deleted);
            assert!(skribe.get(&base_id).is_none());
        }
    }
}
