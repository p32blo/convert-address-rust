use address::{
    models::address::Address,
    models::address_nf_z10_01::NF_Z10_011,
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
        name: String,
        street: String,
        post_code: String,
        city: String,
        country: String,
    },
    /// Update a new address
    Update {
        id: Uuid,
        #[clap(long)]
        name: Option<String>,
        #[clap(long)]
        street: Option<String>,
        #[clap(long)]
        post_code: Option<String>,
        #[clap(long)]
        city: Option<String>,
        #[clap(long)]
        country: Option<String>,
    },
    /// Retrieve an Address by Id
    Get { id: Uuid },
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
            name,
            street,
            post_code,
            city,
            country,
        } => {
            let address = Address {
                name: name.into(),
                street_name: street.into(),
                post_code,
                town_name: city,
                country,
                ..Default::default()
            };
            let id = repository.save(address).expect("Error Saving");
            println!("Address saved at `{}`!", id);
        }
        Commands::Update {
            id,
            name,
            street,
            post_code,
            city,
            country,
        } => {
            let mut data = repository.get(id).expect("This id does not exist");

            if name.is_some() {
                data.name = name;
            }
            if street.is_some() {
                data.street_name = street;
            }
            if let Some(new_post_code) = post_code {
                data.post_code = new_post_code;
            }
            if let Some(new_city) = city {
                data.town_name = new_city;
            }
            if let Some(new_country) = country {
                data.country = new_country;
            }
            repository.update(id, data.clone()).expect("Error Saving");
            println!(
                "Address saved!\n{}",
                TryInto::<NF_Z10_011>::try_into(data)
                    .expect("might fail")
                    .lines
                    .join("\n")
            );
        }

        Commands::Get { id } => {
            let address = repository.get(id).expect("does not exist");
            println!(
                "{}",
                serde_json::to_string_pretty(&address).expect("might fail")
            );
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
