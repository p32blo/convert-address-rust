use address::{
    models::{
        address::Address, address_iso_20022::ISO_20022,
        address_nf_z10_01_individual::NF_Z10_011_Individual,
    },
    repositories::{address_repository::AddressRepository, json_repository::JsonFileRepository},
};
use clap::{Parser, Subcommand, ValueEnum};
use quick_xml::se::Serializer;
use serde::Serialize;
use std::error::Error;
use std::fs;
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
    Add {
        #[clap(long)]
        name: Option<String>,
        #[clap(long)]
        department: Option<String>,
        #[clap(long)]
        sub_department: Option<String>,
        #[clap(long)]
        street_name: Option<String>,
        #[clap(long)]
        building_number: Option<String>,
        #[clap(long)]
        building_name: Option<String>,
        #[clap(long)]
        floor: Option<String>,
        #[clap(long)]
        post_box: Option<String>,
        #[clap(long)]
        room: Option<String>,
        #[clap(long)]
        post_code: String,
        #[clap(long)]
        town_name: String,
        #[clap(long)]
        town_location_name: Option<String>,
        #[clap(long)]
        district_name: Option<String>,
        #[clap(long)]
        country_sub_division: Option<String>,
        #[clap(long)]
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
    Get {
        id: Uuid,
        #[clap(long)]
        format: Option<Format>,
    },
    /// List all saved addresses
    List,
    /// Delete an address
    Delete { id: Uuid },
    /// Convert between address formats
    Convert {
        file: String,
        #[clap(long)]
        from: Format,
        #[clap(long)]
        to: Format,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Iso,
    Nf,
    Json,
}

fn run_cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut repository = JsonFileRepository::new();

    match cli.command {
        Commands::Add {
            name,
            department,
            sub_department,
            street_name,
            building_number,
            building_name,
            floor,
            post_box,
            room,
            post_code,
            town_name,
            town_location_name,
            district_name,
            country_sub_division,
            country,
        } => {
            let address = Address {
                name,
                department,
                sub_department,
                street_name,
                building_number,
                building_name,
                floor,
                post_box,
                room,
                post_code,
                town_name,
                town_location_name,
                district_name,
                country_sub_division,
                country,
            };
            let id = repository.save(address)?;
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
            let mut data = repository.get(id).ok_or("ID not Found")?;

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
            repository.update(id, data.clone())?;
            println!(
                "Address saved!\n{}",
                TryInto::<NF_Z10_011_Individual>::try_into(data)?
                    .lines
                    .join("\n")
            );
        }

        Commands::Get { id, format } => {
            let address = repository.get(id).ok_or("Address Not Found! ")?;

            let format = format.unwrap_or(Format::Json);

            let b = print_with_format(&address, format)?;
            println!("{}", b);
        }
        Commands::List => {
            let addresses = repository.list();
            println!("{:?}", addresses);
        }

        Commands::Delete { id } => {
            let _ = repository.delete(id);
            println!("Address deleted!");
        }

        Commands::Convert { file, from, to } => {
            let content = fs::read_to_string(file)?;

            let from_parse: Address = {
                match from {
                    Format::Json => serde_json::from_str(&content)?,
                    Format::Iso => quick_xml::de::from_str::<ISO_20022>(&content)?.try_into()?,
                    Format::Nf => content.parse::<NF_Z10_011_Individual>()?.try_into()?,
                }
            };
            let output = print_with_format(&from_parse, to)?;
            println!("{}", output);
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("error: {}", e);
    }
}

fn print_with_format(address: &Address, format: Format) -> Result<String, Box<dyn Error>> {
    let address = address.clone();
    Ok(match format {
        Format::Json => serde_json::to_string_pretty(&address)?,
        Format::Iso => {
            let data = ISO_20022::try_from(address)?;

            let mut buffer = String::new();
            let mut serializer = Serializer::new(&mut buffer);
            serializer.indent(' ', 4);
            data.serialize(serializer)?;

            buffer
        }
        Format::Nf => NF_Z10_011_Individual::try_from(address)?.lines.join("\n"),
    })
}
