#![allow(non_camel_case_types, non_snake_case)]

use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
pub struct ParseNFError;

#[derive(Debug, Default)]
pub struct NF_Z10_011 {
    ligne1: String,
    ligne2: String,
    ligne3: String,
    ligne4: String,
    ligne5: String,
    ligne6: String,
    ligne7: String,
}

impl FromStr for NF_Z10_011 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = NF_Z10_011::default();

        for (i, line) in s.lines().enumerate() {
            let line = line.to_string();
            match i + 1 {
                1 => result.ligne1 = line,
                2 => result.ligne2 = line,
                3 => result.ligne3 = line,
                4 => result.ligne4 = line,
                5 => result.ligne5 = line,
                6 => result.ligne6 = line,
                7 => result.ligne7 = line,
                _ => {}
            }
        }

        Ok(result)
    }
}

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
            Dept: value.ligne1,
            ..ISO_20022::default()
        })
    }
}

impl FromStr for ISO_20022 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}
