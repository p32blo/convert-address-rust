use super::address_repository::AddressRepository;
use crate::common::Result;
use crate::models::address::Address;
use serde_json;
use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use uuid::Uuid;

const FILE_PATH: &str = "addresses.json";

/// A repository implementation that stores address data in a JSON file.
///
/// This repository reads and writes to a FILE in JSON to persist address data.
pub struct JsonFileRepository;

pub type Storage = HashMap<Uuid, Address>;

impl JsonFileRepository {
    /// Creates a new instance of `JsonFileRepository`
    pub fn new() -> Self {
        Self
    }

    /// Reads the address storage from a JSON file
    fn read() -> Storage {
        let mut file = match File::open(FILE_PATH) {
            Ok(file) => file,
            Err(_) => return HashMap::new(), // Return empty on any error
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }

    /// Writes the given address storage to a JSON file
    fn write(data: &Storage) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(FILE_PATH, json)?;
        Ok(())
    }
}

impl AddressRepository for JsonFileRepository {
    fn save(&mut self, address: &Address) -> Result<Uuid> {
        let mut storage = Self::read();
        let id = Uuid::new_v4();
        storage.insert(id, address.clone());
        Self::write(&storage)?;
        Ok(id)
    }

    fn update(&mut self, id: Uuid, new_address: &Address) -> Result<()> {
        let mut storage = Self::read();
        match storage.entry(id) {
            Occupied(mut e) => {
                e.insert(new_address.clone());
                Self::write(&storage)
            }
            _ => Err("Address not found".into()),
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<()> {
        let mut storage = Self::read();
        match storage.remove(&id) {
            Some(_) => Self::write(&storage),
            _ => Err("Address not found".into()),
        }
    }

    fn get(&self, id: Uuid) -> Option<Address> {
        let storage = Self::read();
        storage.get(&id).cloned()
    }

    fn list(&self) -> Vec<Address> {
        let storage = Self::read();
        storage.values().cloned().collect()
    }
}
