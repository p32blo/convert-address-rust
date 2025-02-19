use serde::Deserialize;
use serde::Serialize;

use serde_xml_rs;
use std::error::Error;
use std::str::FromStr;

use super::address_nf_z10_01::NF_Z10_011;
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
// serde rename struct
pub struct ISO_20022 {
    Dept: String,
    SubDept: String,
    StrtNm: String,
    BldgNb: String,
    Flr: String,
    PstBx: String,
    Room: String,
    PstCd: String,
    TwnNm: String,
    TwnLctnNm: String,
    StrctNm: String,
    CtrySubDvsn: String,
    Ctry: String,
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
            <ISO_20022>
                <Dept></Dept>
                <SubDept></SubDept>,
                <StrtNm></StrtNm>,
                <BldgNb></BldgNb>,
                <Flr></Flr>,
                <PstBx></PstBx>,
                <Room></Room>,
                <PstCd></PstCd>,
                <TwnNm></TwnNm>,
                <TwnLctnNm></TwnLctnNm>,
                <StrctNm></StrctNm>,
                <CtrySubDvsn></CtrySubDvsn>,
                <Ctry></Ctry>,
            </ISO_20022>
        "#;

        let parse: ISO_20022 = content.parse().expect("Cannot parse XML!");
        assert_eq!(parse, ISO_20022::default());
    }
}
