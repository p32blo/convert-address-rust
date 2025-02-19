pub fn country_to_alpha2(country: &str) -> &str {
    match country {
        "France" => "FR",
        "Portugal" => "PT",
        "Germany" => "DE",
        _ => todo!(),
    }
}

pub fn alpha2_to_country(alpha2: &str) -> &str {
    match alpha2 {
        "FR" => "France",
        "PT" => "Portugal",
        "DE" => "Germany",
        _ => todo!(),
    }
}
