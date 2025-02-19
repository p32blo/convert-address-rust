use address::{
    models::address::Address,
    repositories::{address_repository::AddressRepository, json_repository::JsonFileRepository},
};
use clap::{Parser, Subcommand};
use uuid::Uuid;

/// CLI for managing postal addresses
#[derive(Parser)]
#[command(name = "address-cli")]
#[command(about = "A CLI to convert and store postal addresses")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Save a new address
    Save {
        street: String,
        postal_code: String,
        city: String,
        country: String,
        building_number: Option<String>,
    },

    /// List all saved addresses
    List,

    /// Delete an address
    Delete { id: Uuid },
}

fn main() {
    let cli = Cli::parse();
    let mut repository = JsonFileRepository::new();

    match cli.command {
        Commands::Save {
            street,
            building_number,
            postal_code,
            city,
            country,
        } => {
            let address = Address::default();
            //  {
            //     street,
            //     building_number,
            //     postal_code,
            //     city,
            //     country,
            //     subdivision: None,
            // };
            let id = repository.save(address).expect("Error Saving");
            println!("Address saved at `{}`!", id);
        }
        Commands::List => {
            let addresses = repository.list();
            println!("{:?}", addresses);
        }

        Commands::Delete { id } => {
            let _ = repository.delete(id);
            println!("Address deleted!");
        }
    }
}
