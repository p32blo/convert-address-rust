use serde::Deserialize;
use serde::Serialize;

use serde_xml_rs;
use std::error::Error;
use std::str::FromStr;

use super::address::Address;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename = "PstlAdr")]
// serde rename struct
pub struct ISO_20022 {
    /// Department
    pub Dept: String,
    /// Sub Department
    pub SubDept: String,
    /// Street Name
    pub StrtNm: String,
    /// BUilding Number
    pub BldgNb: String,
    /// Building Name
    pub BldgNm: String,
    /// Floor
    pub Flr: String,
    /// Post Box
    pub PstBx: String,
    /// Room
    pub Room: String,
    /// Post Code
    pub PstCd: String,
    /// Town Name
    pub TwnNm: String,
    /// Town Location Name
    pub TwnLctnNm: String,
    /// District Name
    pub DstrctNm: String,
    /// Country Sub Division
    pub CtrySubDvsn: String,
    /// Country
    pub Ctry: String,
}

impl FromStr for ISO_20022 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

impl TryFrom<Address> for ISO_20022 {
    type Error = Box<dyn Error>;

    fn try_from(value: Address) -> Result<Self, Self::Error> {
        Ok(ISO_20022 {
            Dept: value.department.unwrap_or_default(),
            SubDept: value.sub_department.unwrap_or_default(),
            StrtNm: value.street_name.unwrap_or_default(),
            BldgNb: value.building_number.unwrap_or_default(),
            BldgNm: value.building_name.unwrap_or_default(),
            Flr: value.floor.unwrap_or_default(),
            PstBx: value.post_box.unwrap_or_default(),
            Room: value.room.unwrap_or_default(),
            PstCd: value.post_code,
            TwnNm: value.town_name,
            TwnLctnNm: value.town_location_name.unwrap_or_default(),
            DstrctNm: value.district_name.unwrap_or_default(),
            CtrySubDvsn: value.country_sub_division.unwrap_or_default(),
            Ctry: value.country,
        })
    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() {
        let content = r#"
            <PstlAdr>
                <Dept></Dept>
                <SubDept></SubDept>,
                <StrtNm></StrtNm>,
                <BldgNb></BldgNb>,
                <BldgNm></BldgNm>,
                <Flr></Flr>,
                <PstBx></PstBx>,
                <Room></Room>,
                <PstCd></PstCd>,
                <TwnNm></TwnNm>,
                <TwnLctnNm></TwnLctnNm>,
                <DstrctNm></DstrctNm>,
                <CtrySubDvsn></CtrySubDvsn>,
                <Ctry></Ctry>,
            </PstlAdr>
        "#;

        let parse: ISO_20022 = content.parse().expect("Cannot parse XML!");
        assert_eq!(parse, ISO_20022::default());
    }
}
