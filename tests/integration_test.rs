use address::{
    models::address::Address,
    repositories::{
        address_repository::AddressRepository, in_memory_repository::InMemoryRepository,
    },
};

#[test]
fn test_repository() {
    let mut repo = InMemoryRepository::new();

    let a1 = Address {
        street_name: "RUE DES FLEURS".to_string().into(),
        building_number: Some("25D".to_string()),
        post_code: "33500".to_string(),
        town_name: "Paris".to_string(),
        country: "FR".to_string(),
        ..Default::default()
    };
    let a2 = Address {
        street_name: "VILLA BEAU SOLEIL".to_string().into(),
        building_number: Some("LE VILLAGE".to_string()),
        post_code: "82500".to_string(),
        town_name: "AUTERIVE".to_string(),
        country: "FR".to_string(),
        ..Default::default()
    };

    // Test Save
    let id = repo.save(&a1).expect("failed to save");

    // Test Get
    let found = repo.get(id);
    assert_eq!(Some(&a1), found.as_ref());

    // Add another element
    let id = repo.save(&a2).expect("failed to save");

    // Test Delete
    repo.delete(id).expect("failed to delete");

    // test List
    let list = repo.list();
    assert_eq!(vec![a1], list);
}
