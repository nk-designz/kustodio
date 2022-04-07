use super::traits::{Storage, StorageError};
use bloomfilter::Bloom;
use hashbrown::HashMap;
use serde::Deserialize;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Memory<Key, Value> {
    bloom_filter: Arc<RwLock<Bloom<Key>>>,
    hash_map: Arc<RwLock<HashMap<Key, Value>>>,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub bitmap_size: usize,
    pub items_count: usize,
}

impl<Key, Value> Memory<Key, Value> {
    pub fn new(config: Config) -> Self {
        Memory {
            bloom_filter: Arc::new(RwLock::new(Bloom::new(
                config.bitmap_size,
                config.items_count,
            ))),
            hash_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
use std::fmt::Debug;
impl<Key, Value> Storage<Key, Value> for Memory<Key, Value>
where
    Key: Hash + Eq + Clone + Debug,
    Value: Clone + Debug,
{
    fn clone_safe(&self) -> Self {
        Memory {
            bloom_filter: Arc::clone(&self.bloom_filter),
            hash_map: Arc::clone(&self.hash_map),
        }
    }
    fn probe(&self, key: Key) -> bool {
        match self.bloom_filter.read().unwrap().check(&key) {
            true => self.hash_map.read().unwrap().contains_key(&key),
            false => false,
        }
    }

    fn set(&self, key: Key, value: Value) -> Result<Option<Value>, StorageError> {
        match self.get(key.clone()) {
            Err(_) => {
                self.bloom_filter.write().unwrap().set(&key);
                match self.hash_map.write().unwrap().try_insert(key, value) {
                    Ok(_) => Ok(None),
                    Err(err) => Err(anyhow::Error::msg(format!(
                        "Storage Occupied! ({})",
                        err.to_string()
                    ))),
                }
            }
            Ok(old) => {
                self.hash_map.write().unwrap().remove(&key);
                match self.hash_map.write().unwrap().try_insert(key, value) {
                    Ok(_) => Ok(Some(old)),
                    Err(err) => Err(anyhow::Error::msg(format!(
                        "Storage Occupied! ({})",
                        err.to_string()
                    ))),
                }
            }
        }
    }

    fn get(&self, key: Key) -> Result<Value, StorageError> {
        match self.probe(key.clone()) {
            true => match self.hash_map.read().unwrap().get(&key) {
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
                self.hash_map.write().unwrap().remove(&key);
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

    fn list(&self) -> Result<Vec<(Key, Value)>, StorageError> {
        Ok(self
            .hash_map
            .read()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect())
    }
}
