use models::address::Address;
use repositories::{
    address_repository::AddressRepository, in_memory_repository::InMemoryRepository,
};

mod models;
mod repositories;

fn main() {
    let mut repo = InMemoryRepository::new();

    let address = Address {
        street: "RUE DES FLEURS".to_string(),
        building_number: Some("25D".to_string()),
        postal_code: "33500".to_string(),
        city: "Paris".to_string(),
        country: "FR".to_string(),
        subdivision: None,
    };

    let id = repo.save(address.clone()).unwrap();
    println!("Saved address at `{}`!", id);

    let found = repo.list();
    println!("All Addresses: {:?}", found);
}
