use std::ops::Index;
use std::str::FromStr;

use std::error::Error;

use crate::common::{alpha2_to_country, Result};

use super::address::Address;
use super::validate::Validate;

const MAX_LENGTH: usize = 38;

#[derive(Debug, Default, PartialEq)]
pub struct NF_Z10_011_Individual {
    pub lines: [String; 7],
}

impl Validate for NF_Z10_011_Individual {
    fn validate(&self) -> Result<()> {
        for (i, line) in self.lines.iter().enumerate() {
            if line.len() > MAX_LENGTH {
                return Err(
                    format!("Line {} must have less than {MAX_LENGTH} chars", i + 1).into(),
                );
            }
        }

        Ok(())
    }
}

impl Index<u8> for NF_Z10_011_Individual {
    type Output = str;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0..=7 => self.lines[index as usize - 1].as_ref(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for NF_Z10_011_Individual {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut res = NF_Z10_011_Individual::default();

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

impl TryFrom<Address> for NF_Z10_011_Individual {
    type Error = Box<dyn Error>;

    fn try_from(value: Address) -> Result<Self> {
        let country = alpha2_to_country(&value.country).to_string();
        Ok(NF_Z10_011_Individual {
            lines: [
                value.name.unwrap_or_default(),
                value.room.unwrap_or_default(),
                value.floor.unwrap_or_default(),
                value.street_name.unwrap_or_default(),
                value.town_location_name.unwrap_or_default(),
                format!("{} {}", value.post_code, value.town_name)
                    .trim()
                    .to_string(),
                country,
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_french_individual_example1() {
        let result = Address {
            name: "Monsieur Jean DURAND".to_string().into(),
            street_name: "25D RUE DES FLEURS".to_string().into(),
            post_code: "33500".to_string(),
            town_name: "LIBOURNE".to_string(),
            country: "FR".to_string(),
            ..Default::default()
        };

        let nf_address = NF_Z10_011_Individual {
            lines: [
                "Monsieur Jean DURAND".to_string(),
                "".to_string(),
                "".to_string(),
                "25D RUE DES FLEURS".to_string(),
                "".to_string(),
                "33500 LIBOURNE".to_string(),
                "FRANCE".to_string(),
            ],
        };
        let addr: NF_Z10_011_Individual = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }

    #[test]
    fn test_from_french_individual_example2() {
        let result = Address {
            name: "Monsieur Jean DELHOURME".to_string().into(),
            street_name: "25 RUE DE L’EGLISE".to_string().into(),
            floor: "Entrée A Bâtiment Jonquille".to_string().into(),
            room: "Chez Mireille COPEAU Appartement 2".to_string().into(),
            post_code: "33380".to_string(),
            town_name: "MIOS".to_string(),
            town_location_name: "CAUDOS".to_string().into(),
            country: "FR".to_string(),
            ..Default::default()
        };

        let nf_address = NF_Z10_011_Individual {
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

        let addr: NF_Z10_011_Individual = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }

    #[test]
    fn test_from_french_individual_example3_v1() {
        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            street_name: "VILLA BEAU SOLEIL".to_string().into(),
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            country: "FR".to_string(),
            ..Default::default()
        };

        let nf_address = NF_Z10_011_Individual {
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

        let addr: NF_Z10_011_Individual = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }

    #[test]
    fn test_from_french_individual_example3_v2() {
        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            floor: "VILLA BEAU SOLEIL".to_string().into(),
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            country: "FR".to_string(),
            ..Default::default()
        };

        let nf_address = NF_Z10_011_Individual {
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

        let addr: NF_Z10_011_Individual = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }
}
