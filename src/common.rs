pub fn country_to_alpha2(country: &str) -> &str {
    match dbg!(country.to_uppercase().as_ref()) {
        "FRANCE" => "FR",
        "PORTUGAL" => "PT",
        "GERMANY" => "DE",
        _ => todo!(),
    }
}

pub fn alpha2_to_country(alpha2: &str) -> &str {
    match alpha2.to_uppercase().as_ref() {
        "FR" => "France",
        "PT" => "Portugal",
        "DE" => "Germany",
        _ => todo!(),
    }
}
