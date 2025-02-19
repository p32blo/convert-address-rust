use std::ops::Index;
use std::str::FromStr;

use std::error::Error;

use crate::common::{alpha2_to_country, Result};

use super::address::Address;
use super::validate::Validate;

const MAX_LENGTH: usize = 38;

#[derive(Debug, Default, PartialEq)]
pub struct NF_Z10_011_Enterprise {
    pub lines: [String; 7],
}

impl Validate for NF_Z10_011_Enterprise {
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

impl Index<u8> for NF_Z10_011_Enterprise {
    type Output = str;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0..=7 => self.lines[index as usize - 1].as_ref(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for NF_Z10_011_Enterprise {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut res = NF_Z10_011_Enterprise::default();

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

impl TryFrom<Address> for NF_Z10_011_Enterprise {
    type Error = Box<dyn Error>;

    fn try_from(value: Address) -> Result<Self> {
        let country = alpha2_to_country(&value.country).to_string();
        Ok(NF_Z10_011_Enterprise {
            lines: [
                value.name.unwrap_or_default(),
                value.department.unwrap_or_default(),
                value.floor.unwrap_or_default(),
                value.street_name.unwrap_or_default(),
                format!(
                    "{} {}",
                    value.post_box.unwrap_or_default(),
                    value.town_location_name.unwrap_or_default()
                )
                .trim()
                .to_string(),
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
    fn test_from_french_enterprise_example1() {
        let result = Address {
            name: "DURAND SA".to_string().into(),
            department: "Service achat".to_string().into(),
            street_name: "22BIS RUE DES FLEURS".to_string().into(),
            floor: "Zone industrielle de la Ballastrierre Ouest"
                .to_string()
                .into(),
            post_box: "BP 40122".to_string().into(),
            post_code: "33506".to_string().into(),
            town_name: "LIBOURNE CEDEX".to_string().into(),
            country: "FR".to_string().into(),
            ..Default::default()
        };

        let nf_address = NF_Z10_011_Enterprise {
            lines: [
                "DURAND SA".to_string(),
                "Service achat".to_string(),
                "Zone industrielle de la Ballastrierre Ouest".to_string(),
                "22BIS RUE DES FLEURS".to_string(),
                "BP 40122".to_string(),
                "33506 LIBOURNE CEDEX".to_string(),
                "FRANCE".to_string(),
            ],
        };
        let addr: NF_Z10_011_Enterprise = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }

    #[test]
    fn test_from_french_enterprise_example2() {
        let result = Address {
            name: "Société DUPONT".to_string().into(),
            department: "Mademoiselle Lucie MARTIN".to_string().into(),
            street_name: "56 RUE EMILE ZOLA".to_string().into(),
            floor: "Résidence des Capucins Bâtiment Quater".to_string().into(),
            post_box: "BP 90432".to_string().into(),
            post_code: "34092".to_string().into(),
            town_name: "MONTPELLIER CEDEX 5".to_string(),
            town_location_name: "MONTFERRIER SUR LEZ".to_string().into(),
            country: "FR".to_string(),

            ..Default::default()
        };

        let nf_address = NF_Z10_011_Enterprise {
            lines: [
                "Société DUPONT".to_string(),
                "Mademoiselle Lucie MARTIN".to_string(),
                "Résidence des Capucins Bâtiment Quater".to_string(),
                "56 RUE EMILE ZOLA".to_string(),
                "BP 90432 MONTFERRIER SUR LEZ".to_string(),
                "34092 MONTPELLIER CEDEX 5".to_string(),
                "FRANCE".to_string(),
            ],
        };

        let addr: NF_Z10_011_Enterprise = result.try_into().expect("error");
        assert_eq!(addr, nf_address);
    }
}
