use serde::Deserialize;
use serde::Serialize;

use crate::common::country_to_alpha2;

use super::address_iso_20022::ISO_20022;
use super::address_nf_z10_01::NF_Z10_011;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub name: Option<String>,
    pub department: Option<String>,
    pub sub_department: Option<String>,
    pub street_name: String,
    pub building_number: Option<String>,
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

impl TryFrom<NF_Z10_011> for Address {
    type Error = ();

    fn try_from(value: NF_Z10_011) -> Result<Self, Self::Error> {
        let name = Some(value[1].to_string());
        let street_name = value[4].to_string();
        let country = country_to_alpha2(&value[7]).to_string();

        let (post_code, town_name) = match value[6].split_once(' ') {
            Some((code, city)) => (code.to_string(), city.to_string()),
            _ => ("".to_string(), "".to_string()),
        };
        Ok(Address {
            name,
            street_name,
            country,
            post_code,
            town_name,
            ..Address::default()
        })
    }
}

impl TryFrom<ISO_20022> for Address {
    type Error = ();

    fn try_from(value: ISO_20022) -> Result<Self, Self::Error> {
        Ok(Address {
            name: None,
            department: Some(value.Dept),
            sub_department: Some(value.SubDept),
            street_name: value.StrtNm,
            building_number: Some(value.BldgNb),
            floor: Some(value.Flr),
            post_box: Some(value.PstBx),
            room: Some(value.Room),
            post_code: value.PstCd,
            town_name: value.TwnNm,
            town_location_name: Some(value.TwnLctnNm),
            district_name: Some(value.DstrctNm),
            country_sub_division: Some(value.CtrySubDvsn),
            country: value.Ctry,
        })
    }
}

#[cfg(test)]
mod tests {
    use clap::builder::NonEmptyStringValueParser;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_from_french() {
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
            name: Some("Monsieur Jean DURAND".to_string()),
            department: None,
            sub_department: None,
            street_name: "25D RUE DES FLEURS".to_string(),
            building_number: None,
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
}
