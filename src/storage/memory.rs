use super::traits::{Storage, StorageError};
use bloomfilter::Bloom;
use hashbrown::HashMap;
use serde::Deserialize;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Memory<Key, Value> {
    bloom_filter: Arc<Mutex<Bloom<Key>>>,
    hash_map: Arc<Mutex<HashMap<Key, Value>>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub bitmap_size: usize,
    pub items_count: usize,
}

impl Config {
    fn default() -> Self {
        Config {
            bitmap_size: 6000,
            items_count: 6000,
        }
    }
}

impl<Key, Value> Memory<Key, Value> {
    pub fn new(config: Config) -> Self {
        Memory {
            bloom_filter: Arc::new(Mutex::new(Bloom::new(
                config.bitmap_size,
                config.items_count,
            ))),
            hash_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl<Key, Value> Storage<Key, Value, Config> for Memory<Key, Value>
where
    Key: Hash + Eq + Clone,
    Value: Clone,
{
    fn clone_safe(&self) -> Self {
        Memory {
            bloom_filter: Arc::clone(&self.bloom_filter),
            hash_map: Arc::clone(&self.hash_map),
        }
    }
    fn probe(&self, key: Key) -> bool {
        match self.bloom_filter.lock().unwrap().check(&key) {
            true => self.hash_map.lock().unwrap().contains_key(&key),
            false => false,
        }
    }

    fn set(&self, key: Key, value: Value) -> Result<Option<Value>, StorageError> {
        match self.get(key.clone()) {
            Err(_) => {
                self.bloom_filter.lock().unwrap().set(&key);
                match self.hash_map.lock().unwrap().try_insert(key, value) {
                    Ok(_) => Ok(None),
                    Err(_) => Err(anyhow::Error::msg("Storage Occupied!")),
                }
            }
            Ok(old) => match self.hash_map.lock().unwrap().try_insert(key, value) {
                Ok(_) => Ok(Some(old)),
                Err(_) => Err(anyhow::Error::msg("Storage Occupied!")),
            },
        }
    }

    fn get(&self, key: Key) -> Result<Value, StorageError> {
        match self.probe(key.clone()) {
            true => match self.hash_map.lock().unwrap().get(&key) {
                Some(value) => Ok(value.clone()),
                None => Err(anyhow::Error::msg("Key not found.")),
            },
            false => Err(anyhow::Error::msg("Key not found.")),
        }
    }

    fn remove(&self, key: Key) -> Result<Value, StorageError> {
        match self.get(key.clone()) {
            Ok(value_ref) => {
                let value = value_ref.clone();
                self.hash_map.lock().unwrap().remove(&key);
                Ok(value)
            }
            err => err,
        }
    }

    fn swap(&self, key: Key, value_reference: &mut Value) -> Result<(), StorageError> {
        if let Some(old) = self.set(key, value_reference.clone())? {
            *value_reference = old;
        }
        Ok(())
    }
}
