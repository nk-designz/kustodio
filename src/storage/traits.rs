pub type StorageError = anyhow::Error;

pub trait Storage<Key, Value> {
    fn clone_safe(&self) -> Self;
    fn probe(&self, key: Key) -> bool;
    fn set(&self, key: Key, value: Value) -> Result<Option<Value>, StorageError>;
    fn get(&self, key: Key) -> Result<Value, StorageError>;
    fn remove(&self, key: Key) -> Result<Value, StorageError>;
    fn swap(&self, key: Key, value_reference: &mut Value) -> Result<(), StorageError>;
}
