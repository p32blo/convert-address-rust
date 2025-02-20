use uuid::Uuid;

use crate::common::Result;

use crate::models::address::Address;

pub trait AddressRepository {
    /// Saves a new address and returns its unique identifier
    ///
    /// # Arguments
    ///
    /// * `address` - A reference to the `Address` to be saved
    ///
    /// # Returns
    ///
    /// * `Result<Uuid>` - The UUID of the saved address
    ///
    fn save(&mut self, address: &Address) -> Result<Uuid>;

    /// Updates an existing address identified by its UUID
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the address to be updated
    /// * `new_address` - The updated `Address` data
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An empty `Result` indicating success or an error if the update fails
    ///
    fn update(&mut self, id: Uuid, new_address: &Address) -> Result<()>;

    /// Deletes an address identified by its UUID
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the address to be deleted
    ///
    /// # Returns
    ///
    /// * `Result<()>` - An empty `Result` indicating success or an error if deletion fails
    ///
    fn delete(&mut self, id: Uuid) -> Result<()>;

    /// Retrieves an address by its UUID
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the address to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<Address>` - The address when available
    ///
    fn get(&self, id: Uuid) -> Option<Address>;

    /// Lists all stored addresses
    ///
    /// # Returns
    ///
    /// * `Vec<Address>` - List of addresses
    ///
    fn list(&self) -> Vec<Address>;
}
