use std::ops::Index;
use std::str::FromStr;

use std::error::Error;

use clap::ValueHint;

use crate::common::alpha2_to_country;

use super::address::Address;

#[derive(Debug, Default)]
pub struct NF_Z10_011 {
    pub lines: [String; 7],
}

impl Index<u8> for NF_Z10_011 {
    type Output = str;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0..=7 => self.lines[index as usize - 1].as_ref(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for NF_Z10_011 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = NF_Z10_011::default();

        let lines: Vec<String> = s.lines().map(|x| x.to_string()).collect();

        if lines.len() > 7 {
            return Err("Input must contain no more than 7 lines".into());
        }

        for (i, line) in lines.into_iter().enumerate() {
            res.lines[i] = line;
        }

        Ok(res)
    }
}

impl TryFrom<Address> for NF_Z10_011 {
    type Error = Box<dyn Error>;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        let country = alpha2_to_country(&value.country).to_string();
        Ok(NF_Z10_011 {
            lines: [
                value.name.unwrap_or_default(),
                value.room.unwrap_or_default(),
                value.floor.unwrap_or_default(),
                value.street_name.unwrap_or_default(),
                value.town_location_name.unwrap_or_default(),
                format!("{} {}", value.post_code, value.town_name),
                country,
            ],
        })
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_from_french_individual_example1() {
        let nf_address = NF_Z10_011 {
            lines: [
                "Monsieur Jean DURAND".to_string(),
                "".to_string(),
                "".to_string(),
                "25D RUE DES FLEURS".to_string(),
                "".to_string(),
                "33500 LIBOURNE".to_string(),
                "France".to_string(),
            ],
        };

        let result = Address {
            name: "Monsieur Jean DURAND".to_string().into(),
            department: None,
            sub_department: None,
            street_name: "25D RUE DES FLEURS".to_string().into(),
            building_number: None,
            building_name: None,
            floor: None,
            post_box: None,
            room: None,
            post_code: "33500".to_string(),
            town_name: "LIBOURNE".to_string(),
            town_location_name: None,
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }
    #[test]
    fn test_from_french_individual_example2() {
        let nf_address = NF_Z10_011 {
            lines: [
                "Monsieur Jean DELHOURME".to_string(),
                "Chez Mireille COPEAU Appartement 2".to_string(),
                "Entrée A Bâtiment Jonquille".to_string(),
                "25 RUE DE L’EGLISE".to_string(),
                "CAUDOS".to_string(),
                "33380 MIOS".to_string(),
                "FRANCE".to_string(),
            ],
        };

        let result = Address {
            name: "Monsieur Jean DELHOURME".to_string().into(),
            department: None,
            sub_department: None,
            street_name: "25 RUE DE L’EGLISE".to_string().into(),
            building_number: None,
            building_name: None,
            floor: "Entrée A Bâtiment Jonquille".to_string().into(),
            post_box: None,
            room: "Chez Mireille COPEAU Appartement 2".to_string().into(),
            post_code: "33380".to_string(),
            town_name: "MIOS".to_string(),
            town_location_name: "CAUDOS".to_string().into(),
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_individual_example3_v1() {
        let nf_address = NF_Z10_011 {
            lines: [
                "Madame Isabelle RICHARD".to_string(),
                "".to_string(),
                "".to_string(),
                "VILLA BEAU SOLEIL".to_string(),
                "LE VILLAGE".to_string(),
                "82500 AUTERIVE".to_string(),
                "FRANCE".to_string(),
            ],
        };

        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            department: None,
            sub_department: None,
            street_name: "VILLA BEAU SOLEIL".to_string().into(),
            building_number: None,
            building_name: None,
            floor: None,
            post_box: None,
            room: None,
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_individual_example3_v2() {
        let nf_address = NF_Z10_011 {
            lines: [
                "Madame Isabelle RICHARD".to_string(),
                "".to_string(),
                "VILLA BEAU SOLEIL".to_string(),
                "".to_string(),
                "LE VILLAGE".to_string(),
                "82500 AUTERIVE".to_string(),
                "FRANCE".to_string(),
            ],
        };

        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            department: None,
            sub_department: None,
            street_name: None,
            building_number: None,
            building_name: None,
            floor: "VILLA BEAU SOLEIL".to_string().into(),
            post_box: None,
            room: None,
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }
}
