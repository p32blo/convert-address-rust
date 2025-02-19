use std::collections::HashMap;
use std::error::Error;

use uuid::Uuid;

use crate::models::address::Address;

use super::address_repository::AddressRepository;

#[derive(Debug, Default)]
pub struct InMemoryRepository {
    storage: HashMap<Uuid, Address>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl AddressRepository for InMemoryRepository {
    fn save(&mut self, address: Address) -> Result<Uuid, Box<dyn Error>> {
        let id = Uuid::new_v4();
        self.storage.insert(id, address);
        Ok(id)
    }

    fn update(&mut self, id: Uuid, new_address: Address) -> Result<(), Box<dyn Error>> {
        if self.storage.contains_key(&id) {
            self.storage.insert(id, new_address);
            Ok(())
        } else {
            Err("Address not found".into())
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<(), Box<dyn Error>> {
        if self.storage.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Address not found".into())
        }
    }

    fn get(&self, id: Uuid) -> Option<&Address> {
        self.storage.get(&id)
    }

    fn list(&self) -> Vec<Address> {
        self.storage.values().cloned().collect()
    }
}
