use crate::{Property, Result as SageResult};
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Authentication {
    pub method: String,
    pub data: Vec<u8>,
}

impl Authentication {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = Property::AuthenticationMethod(self.method).encode(writer)?;
        if !self.data.is_empty() {
            n_bytes += Property::AuthenticationData(self.data).encode(writer)?;
        }
        Ok(n_bytes)
    }
}

#[cfg(test)]
mod unit {

    use super::*;

    #[test]
    fn encode() {
        let mut result = Vec::new();
        let test_data = Authentication {
            method: "Willow".into(),
            data: vec![0x0D, 0x15, 0xEA, 0x5E],
        };

        assert_eq!(test_data.write(&mut result).unwrap(), 16);
        assert_eq!(
            result,
            vec![21, 0, 6, 87, 105, 108, 108, 111, 119, 22, 0, 4, 13, 21, 234, 94]
        );
    }
}
