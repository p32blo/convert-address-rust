use serde::Deserialize;
use serde::Serialize;

use serde_xml_rs;
use std::error::Error;
use std::str::FromStr;

use super::address_nf_z10_01::NF_Z10_011;
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

impl TryFrom<NF_Z10_011> for ISO_20022 {
    type Error = ();

    fn try_from(value: NF_Z10_011) -> Result<Self, Self::Error> {
        Ok(ISO_20022 {
            Dept: value.lines[0].clone(),
            ..ISO_20022::default()
        })
    }
}

impl FromStr for ISO_20022 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_xml_rs::from_str(s).map_err(|e| Box::new(e) as Box<dyn Error>)
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
