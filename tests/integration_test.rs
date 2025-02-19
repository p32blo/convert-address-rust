use address::{
    models::address::Address,
    repositories::{
        address_repository::AddressRepository, in_memory_repository::InMemoryRepository,
    },
};

// fn get_test_addresses() -> Vec<Address> {
//     vec![
//         Address {
//             street: "RUE DES FLEURS".to_string(),
//             building_number: Some("25D".to_string()),
//             postal_code: "33500".to_string(),
//             city: "Paris".to_string(),
//             country: "FR".to_string(),
//             subdivision: None,
//         },
//         Address {
//             street: "VILLA BEAU SOLEIL".to_string(),
//             building_number: Some("LE VILLAGE".to_string()),
//             postal_code: "82500".to_string(),
//             city: "AUTERIVE".to_string(),
//             country: "FR".to_string(),
//             subdivision: None,
//         },
//     ]
// }

// #[test]
// fn test_repository() {
//     let mut repo = InMemoryRepository::new();

//     let addrs = get_test_addresses();

//     // Test Save
//     let id = repo.save(addrs[0].clone()).expect("failed to save");

//     // Test Get
//     let found = repo.get(id);
//     assert_eq!(Some(&addrs[0]), found.as_ref());

//     // Add another element
//     let id = repo.save(addrs[1].clone()).expect("failed to save");

//     // Test Delete
//     repo.delete(id).expect("failed to delete");

//     // test List
//     let list = repo.list();
//     assert_eq!(vec![addrs[0].clone()], list);
// }
