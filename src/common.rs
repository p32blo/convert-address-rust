use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn country_to_alpha2(country: &str) -> &str {
    match country.to_uppercase().as_ref() {
        "FRANCE" => "FR",
        "PORTUGAL" => "PT",
        "GERMANY" => "DE",
        _ => todo!("Implement more countries"),
    }
}

pub fn alpha2_to_country(alpha2: &str) -> &str {
    match alpha2.to_uppercase().as_ref() {
        "FR" => "FRANCE",
        "PT" => "PORTUGAL",
        "DE" => "GERMANY",
        _ => todo!("Implement more countries"),
    }
}
