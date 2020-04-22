use crate::{Encode, Property, Result as SageResult};
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Authentication {
    pub method: String,
    pub data: Vec<u8>,
}

impl Encode for Authentication {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = Property::AuthenticationMethod(self.method).encode(writer)?;
        if !self.data.is_empty() {
            n_bytes += Property::AuthenticationData(self.data).encode(writer)?;
        }
        Ok(n_bytes)
    }
}
