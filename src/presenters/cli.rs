use address::models::address_nf_z10_011_enterprise::NF_Z10_011_Enterprise;
use address::models::validate::Validate;
use address::{
    models::{
        address::Address, address_iso_20022::ISO_20022,
        address_nf_z10_011_individual::NF_Z10_011_Individual,
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
    /// Add a new address
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
        post_code: Option<String>,
        #[clap(long)]
        town_name: Option<String>,
        #[clap(long)]
        town_location_name: Option<String>,
        #[clap(long)]
        district_name: Option<String>,
        #[clap(long)]
        country_sub_division: Option<String>,
        #[clap(long)]
        country: Option<String>,
        #[clap(long)]
        format: Option<Format>,
    },
    /// Add a Address from an existing file
    Save {
        #[clap(long)]
        file: String,
        #[clap(long)]
        from: Format,
        #[clap(long, action)]
        validate: bool,
        #[clap(long, action)]
        enterprise: bool,
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
        #[clap(long)]
        file: String,
        #[clap(long)]
        from: Format,
        #[clap(long)]
        to: Format,
        #[clap(long, action)]
        validate: bool,
        #[clap(long, action)]
        enterprise: bool,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    /// ISO 200022
    Iso,
    /// NF Z10-011
    Nf,
    /// Json
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
            let id = repository.save(&address)?;
            eprintln!("Address saved at `{}`!", id);
        }
        Commands::Update {
            id,
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
            format,
        } => {
            let mut data = repository.get(id).ok_or("ID not Found")?;

            if name.is_some() {
                data.name = name;
            }
            if department.is_some() {
                data.department = department;
            }
            if sub_department.is_some() {
                data.sub_department = sub_department;
            }
            if street_name.is_some() {
                data.street_name = street_name;
            }
            if building_number.is_some() {
                data.building_number = building_number;
            }
            if building_name.is_some() {
                data.building_name = building_name;
            }
            if floor.is_some() {
                data.floor = floor;
            }
            if post_box.is_some() {
                data.post_box = post_box;
            }
            if room.is_some() {
                data.room = room;
            }
            if let Some(val) = post_code {
                data.post_code = val;
            }
            if let Some(val) = town_name {
                data.town_name = val;
            }
            if town_location_name.is_some() {
                data.town_location_name = town_location_name;
            }
            if district_name.is_some() {
                data.district_name = district_name;
            }
            if country_sub_division.is_some() {
                data.country_sub_division = country_sub_division;
            }
            if let Some(val) = country {
                data.country = val;
            }

            repository.update(id, &data)?;

            let format = format.unwrap_or(Format::Json);
            let content = str_from_address(&data, format)?;
            println!("{}", content);
        }

        Commands::Save {
            file,
            from,
            validate,
            enterprise,
        } => {
            let content = fs::read_to_string(file)?;
            let address = address_from_str(&content, from, validate, enterprise)?;
            let id = repository.save(&address)?;
            eprintln!("Address saved at `{}`!", id);
        }

        Commands::Get { id, format } => {
            let address = repository
                .get(id)
                .ok_or(format!("Address `{id}` not found! "))?;
            let format = format.unwrap_or(Format::Json);
            let content = str_from_address(&address, format)?;
            println!("{}", content);
        }
        Commands::List => {
            let addresses = serde_json::to_string_pretty(&repository.list())?;
            println!("{}", addresses);
        }

        Commands::Delete { id } => {
            let _ = repository.delete(id);
            eprintln!("Address deleted!");
        }

        Commands::Convert {
            file,
            validate,
            from,
            to,
            enterprise,
        } => {
            let content = fs::read_to_string(file)?;

            let from: Address = address_from_str(&content, from, validate, enterprise)?;
            let output = str_from_address(&from, to)?;

            println!("{}", output);
        }
    }
    Ok(())
}

///
///  Parses an address from a given string and format.
///
fn address_from_str(
    content: &str,
    from: Format,
    validate: bool,
    enterprise: bool,
) -> Result<Address, Box<dyn Error>> {
    Ok(match from {
        Format::Json => serde_json::from_str(&content)?,
        Format::Iso => {
            let iso = quick_xml::de::from_str::<ISO_20022>(&content)?;
            if validate {
                iso.validate()?;
            }
            iso.try_into()?
        }
        Format::Nf => {
            if enterprise {
                let nf = content.parse::<NF_Z10_011_Enterprise>()?;
                if validate {
                    nf.validate()?;
                }
                nf.try_into()?
            } else {
                let nf = content.parse::<NF_Z10_011_Individual>()?;
                if validate {
                    nf.validate()?;
                }
                nf.try_into()?
            }
        }
    })
}

///
///  Converts an address into a formatted string representation.
///
fn str_from_address(address: &Address, format: Format) -> Result<String, Box<dyn Error>> {
    let address = address.clone();
    Ok(match format {
        Format::Json => serde_json::to_string_pretty(&address)?,
        Format::Iso => {
            let data = ISO_20022::try_from(address)?;

            // indent XMl

            let mut buffer = String::new();

            let mut serializer = Serializer::new(&mut buffer);
            serializer.indent(' ', 4);
            data.serialize(serializer)?;

            buffer
        }
        Format::Nf => NF_Z10_011_Individual::try_from(address)?.lines.join("\n"),
    })
}

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("error: {}", e);
    }
}
