use super::address_repository::Result;
use crate::models::address::Address;
use serde_json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use uuid::Uuid;

use super::address_repository::AddressRepository;

const FILE_PATH: &str = "addresses.json";

pub struct JsonFileRepository;

pub type Storage = HashMap<Uuid, Address>;

impl JsonFileRepository {
    pub fn new() -> Self {
        Self
    }

    fn read() -> Storage {
        let mut file = match File::open(FILE_PATH) {
            Ok(file) => file,
            Err(_) => return HashMap::new(), // Return empty on any error
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str(&contents).unwrap_or_else(|_| HashMap::new())
        } else {
            HashMap::new()
        }
    }

    fn write(data: &Storage) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE_PATH)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

impl AddressRepository for JsonFileRepository {
    fn save(&mut self, address: Address) -> Result<Uuid> {
        let mut storage = Self::read();
        let id = Uuid::new_v4();
        storage.insert(id, address);
        Self::write(&storage)?;
        Ok(id)
    }

    fn update(&mut self, id: Uuid, new_address: Address) -> Result<()> {
        let mut storage = Self::read();
        if storage.contains_key(&id) {
            storage.insert(id, new_address);
            Self::write(&storage)
        } else {
            Err("Address not found".into())
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<()> {
        let mut storage = Self::read();
        if storage.remove(&id).is_some() {
            Self::write(&storage)
        } else {
            Err("Address not found".into())
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
