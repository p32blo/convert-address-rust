use std::error::Error;
use uuid::Uuid;

use crate::models::address::Address;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub trait AddressRepository {
    fn save(&mut self, address: Address) -> Result<Uuid>;
    fn update(&mut self, id: Uuid, new_address: Address) -> Result<()>;
    fn delete(&mut self, id: Uuid) -> Result<()>;
    fn get(&self, id: Uuid) -> Option<Address>;
    fn list(&self) -> Vec<Address>;
}
