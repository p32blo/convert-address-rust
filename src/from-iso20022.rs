use address::models::address_iso_20022::ISO_20022;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = args.get(1).expect("Error getting the 1st arg");
    let content = std::fs::read_to_string(filename).expect("Failed to read file");
    let nf: ISO_20022 = content.parse().expect("Error reading");
    dbg!(nf);
}
