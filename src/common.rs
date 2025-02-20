use std::error::Error;

/// A type alias for `Result` to avoid having to write the error type all the time
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Converts a country name to its corresponding ISO 3166-1 alpha-2 code.
///
/// # Arguments
///
/// * `country` - Country name (case insensitive).
///
/// # Returns
///
/// * Two-letter country code.
///
/// # Example
///
/// ```
/// use address::common::country_to_alpha2;
/// let code = country_to_alpha2("France");
/// assert_eq!(code, "FR");
/// ```
pub fn country_to_alpha2(country: &str) -> &str {
    match country.to_uppercase().as_ref() {
        "FRANCE" => "FR",
        "PORTUGAL" => "PT",
        "GERMANY" => "DE",
        _ => todo!("Implement more countries"),
    }
}

/// Converts an ISO 3166-1 alpha-2 country code to its corresponding country name.
///
/// # Arguments
///
/// * `alpha2` - Two-letter country code (case insensitive).
///
/// # Returns
///
/// * Full country name.
///
/// # Example
///
/// ```
/// use address::common::alpha2_to_country;
/// let country = alpha2_to_country("FR");
/// assert_eq!(country, "FRANCE");
/// ```
pub fn alpha2_to_country(alpha2: &str) -> &str {
    match alpha2.to_uppercase().as_ref() {
        "FR" => "FRANCE",
        "PT" => "PORTUGAL",
        "DE" => "GERMANY",
        _ => todo!("Implement more countries"),
    }
}
