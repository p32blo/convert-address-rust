use std::error::Error;

use uuid::Uuid;

use crate::models::address::Address;

pub trait AddressRepository {
    fn save(&mut self, address: Address) -> Result<Uuid, Box<dyn Error>>;
    fn update(&mut self, id: Uuid, new_address: Address) -> Result<(), Box<dyn Error>>;
    fn delete(&mut self, id: Uuid) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: Uuid) -> Option<&Address>;
    fn list(&self) -> Vec<Address>;
}
