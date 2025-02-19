use address::models::{
    address::Address, address_iso_20022::ISO_20022,
    address_nf_z10_01_individual::NF_Z10_011_Individual,
};
use serde_xml_rs::to_string;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    let nf: NF_Z10_011_Individual = content.parse().expect("Error reading");

    let address: Address = nf.try_into().expect("This might fail");
    let iso: ISO_20022 = address.try_into().expect("This will fail");
    println!("{}", to_string(&iso).expect("Failed to convert"));
}
