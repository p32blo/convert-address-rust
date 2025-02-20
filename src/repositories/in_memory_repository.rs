use std::collections::HashMap;

use super::address_repository::AddressRepository;

use crate::{common::Result, models::address::Address};
use std::collections::hash_map::Entry::Occupied;
use uuid::Uuid;

/// An in-memory implementation of the `AddressRepository` trait
#[derive(Debug, Default)]
pub struct InMemoryRepository {
    storage: HashMap<Uuid, Address>,
}

impl InMemoryRepository {
    /// Creates a new instance of `InMemoryRepository`
    pub fn new() -> Self {
        Self::default()
    }
}

impl AddressRepository for InMemoryRepository {
    fn save(&mut self, address: &Address) -> Result<Uuid> {
        let id = Uuid::new_v4();
        self.storage.insert(id, address.clone());
        Ok(id)
    }

    fn update(&mut self, id: Uuid, new_address: &Address) -> Result<()> {
        match self.storage.entry(id) {
            Occupied(mut e) => {
                e.insert(new_address.clone());
                Ok(())
            }
            _ => Err("Address not found".into()),
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<()> {
        match self.storage.remove(&id) {
            Some(_) => Ok(()),
            _ => Err("Address not found".into()),
        }
    }

    fn get(&self, id: Uuid) -> Option<Address> {
        self.storage.get(&id).cloned()
    }

    fn list(&self) -> Vec<Address> {
        self.storage.values().cloned().collect()
    }
}
