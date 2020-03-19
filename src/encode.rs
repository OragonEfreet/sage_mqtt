//! Defines the `Encode` trait and implement it for the MQTT types.

use std::io::{Error, ErrorKind, Write};

const ERROR_MSG_STRING_TOO_LONG: &str = "UTF-8 Type cannot exceed 65,535 bytes";

/// The Encode trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W>(&self, writer: &mut W) -> Result<usize, Error>
    where
        W: Write;
}

impl Encode for &str {
    fn encode<W>(&self, writer: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let len = self.as_bytes().len();
        if len > i16::max_value() as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                ERROR_MSG_STRING_TOO_LONG,
            ));
        }
        writer.write(&(len as u16).to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(2 + len)
    }
}

impl Encode for String {
    fn encode<W>(&self, writer: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        (&self[..]).encode(writer)
    }
}

#[cfg(test)]
mod unit_encode {

    use super::*;

    #[test]
    fn encode_utf8_string_00() {

        let input_data = "A𪛔";
        let expected_result = vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
        let mut stream_test: Vec<u8> = Vec::new();
        input_data.encode(&mut stream_test).unwrap();
        assert_eq!(stream_test, expected_result, "Encoding 'A𪛔' failed");
    }

    #[test]
    fn encore_utf8_string_empty() {
        let input_data = "";
        let expected_result = vec![0x00, 0x00];
        let mut stream_test: Vec<u8> = Vec::new();
        input_data.encode(&mut stream_test).unwrap();
        assert_eq!(stream_test, expected_result, "Encoding of empty string failed");
    }

}
