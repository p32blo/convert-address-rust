use serde::Deserialize;
use serde::Serialize;

use crate::common::country_to_alpha2;

use super::address_iso_20022::ISO_20022;
use super::address_nf_z10_01_enterprise::NF_Z10_011_Enterprise;
use super::address_nf_z10_01_individual::NF_Z10_011_Individual;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub name: Option<String>,
    pub department: Option<String>,
    pub sub_department: Option<String>,
    pub street_name: Option<String>,
    pub building_number: Option<String>,
    pub building_name: Option<String>,
    pub floor: Option<String>,
    pub post_box: Option<String>,
    pub room: Option<String>,
    pub post_code: String,
    pub town_name: String,
    pub town_location_name: Option<String>,
    pub district_name: Option<String>,
    pub country_sub_division: Option<String>,
    pub country: String,
}

impl TryFrom<NF_Z10_011_Individual> for Address {
    type Error = ();

    fn try_from(value: NF_Z10_011_Individual) -> Result<Self, Self::Error> {
        let optional = |x: &str| Some(x).filter(|x| !x.is_empty()).map(|x| x.to_string());

        let name = optional(&value[1]);
        let room = optional(&value[2]);
        let floor = optional(&value[3]);
        let street_name = optional(&value[4]);
        let town_location_name = optional(&value[5]);
        let (post_code, town_name) = value[6]
            .split_once(' ')
            .map(|(code, city)| (code.to_string(), city.to_string()))
            .unwrap_or_default();
        let country = country_to_alpha2(&value[7]).to_string();

        Ok(Address {
            name,
            street_name,
            country,
            post_code,
            town_name,
            town_location_name,
            floor,
            room,
            ..Address::default()
        })
    }
}

impl TryFrom<NF_Z10_011_Enterprise> for Address {
    type Error = ();

    fn try_from(value: NF_Z10_011_Enterprise) -> Result<Self, Self::Error> {
        let optional = |x: &str| Some(x).filter(|x| !x.is_empty()).map(|x| x.to_string());

        let name = optional(&value[1]);
        let department = optional(&value[2]);
        let floor = optional(&value[3]);
        let street_name = optional(&value[4]);

        let (post_box, town_location_name) = {
            let parts: Vec<_> = value[5].splitn(3, ' ').collect();

            match parts.as_slice() {
                [first, second, rest @ ..] => (
                    Some(format!("{} {}", first, second)),
                    Some(rest.join(" ")).filter(|x| !x.is_empty()),
                ),
                _ => (None, None),
            }
        };

        let (post_code, town_name) = value[6]
            .split_once(' ')
            .map(|(code, city)| (code.to_string(), city.to_string()))
            .unwrap_or_default();
        let country = country_to_alpha2(&value[7]).to_string();

        Ok(Address {
            name,
            department,
            street_name,
            floor,
            post_box,
            country,
            post_code,
            town_name,
            town_location_name,

            ..Address::default()
        })
    }
}

impl TryFrom<ISO_20022> for Address {
    type Error = ();

    fn try_from(value: ISO_20022) -> Result<Self, Self::Error> {
        Ok(Address {
            name: None,
            department: value.Dept.into(),
            sub_department: value.SubDept.into(),
            street_name: value.StrtNm.into(),
            building_number: value.BldgNb.into(),
            building_name: value.BldgNm.into(),
            floor: value.Flr.into(),
            post_box: value.PstBx.into(),
            room: value.Room.into(),
            post_code: value.PstCd,
            town_name: value.TwnNm,
            town_location_name: value.TwnLctnNm.into(),
            district_name: value.DstrctNm.into(),
            country_sub_division: value.CtrySubDvsn.into(),
            country: value.Ctry,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::models::address_nf_z10_01_enterprise::NF_Z10_011_Enterprise;

    use super::*;

    #[test]
    fn test_from_french_individual_example1() {
        let nf_address = NF_Z10_011_Individual {
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
            street_name: "25D RUE DES FLEURS".to_string().into(),
            post_code: "33500".to_string(),
            town_name: "LIBOURNE".to_string(),
            country: "FR".to_string(),
            ..Default::default()
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }
    #[test]
    fn test_from_french_individual_example2() {
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
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_individual_example3_v1() {
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

        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            street_name: "VILLA BEAU SOLEIL".to_string().into(),
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            country: "FR".to_string(),
            ..Default::default()
        };
        let addr: Address = nf_address.try_into().expect("error");

        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_individual_example3_v2() {
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

        let result = Address {
            name: "Madame Isabelle RICHARD".to_string().into(),
            floor: "VILLA BEAU SOLEIL".to_string().into(),
            post_code: "82500".to_string(),
            town_name: "AUTERIVE".to_string(),
            town_location_name: "LE VILLAGE".to_string().into(),
            country: "FR".to_string(),
            ..Default::default()
        };

        let addr: Address = nf_address.try_into().expect("error");
        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_enterprise_example1() {
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

        let addr: Address = nf_address.try_into().expect("error");
        assert_eq!(addr, result);
    }

    #[test]
    fn test_from_french_enterprise_example2() {
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

        let addr: Address = nf_address.try_into().expect("error");
        assert_eq!(addr, result);
    }
}
