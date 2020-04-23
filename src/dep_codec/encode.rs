use crate::{BinaryData, Error, Result as SageResult, UTF8String};
use std::io::{Error as IOError, ErrorKind, Write};

/// The `Encode` trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl Encode for UTF8String {
    fn encode<W>(self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let len = self.0.len();
        if len > i16::max_value() as usize {
            return Err(Error::MalformedPacket);
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(self.0.as_bytes())?;
        Ok(2 + len)
    }
}

impl Encode for BinaryData {
    fn encode<W>(self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let len = self.0.len();
        if len > i16::max_value() as usize {
            return Err(IOError::new(ErrorKind::InvalidData, "ERROR_MSG_DATA_TOO_LONG").into());
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(&self.0)?;
        Ok(2 + len)
    }
}

#[cfg(test)]
mod unit_encode {

    use super::*;

    #[test]
    fn encode_utf8string() {
        let mut result = Vec::new();
        assert_eq!(UTF8String::from("A𪛔").encode(&mut result).unwrap(), 7);
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_utf8string_empty() {
        let mut result = Vec::new();
        assert_eq!(UTF8String::default().encode(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[test]
    fn encode_binarydata() {
        let mut result = Vec::new();
        assert_eq!(
            BinaryData(Vec::from("A𪛔".as_bytes()))
                .encode(&mut result)
                .unwrap(),
            7
        );
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_binarydata_empty() {
        let mut result = Vec::new();
        assert_eq!(BinaryData(Vec::new()).encode(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }
}
