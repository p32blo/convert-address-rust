use crate::common::Result;

pub trait Validate {
    fn validate(&self) -> Result<()>;
}
