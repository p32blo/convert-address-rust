use serde::Deserialize;
use serde::Serialize;

use super::address_iso_20022::ISO_20022;
use super::address_nf_z10_01::NF_Z10_011;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub street: String,
    pub building_number: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub subdivision: Option<String>,
}

impl TryFrom<NF_Z10_011> for Address {
    type Error = ();

    fn try_from(value: NF_Z10_011) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<ISO_20022> for Address {
    type Error = ();

    fn try_from(value: ISO_20022) -> Result<Self, Self::Error> {
        todo!()
    }
}
