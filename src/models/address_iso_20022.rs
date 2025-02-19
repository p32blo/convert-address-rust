use serde::Deserialize;
use serde::Serialize;

use serde_xml_rs;
use std::error::Error;
use std::str::FromStr;

use super::address_nf_z10_01::NF_Z10_011;
#[derive(Debug, Default, Serialize, Deserialize)]
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
