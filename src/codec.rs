//! Defines the `Encode` trait and implement it for the MQTT types.

use crate::types::*;
use std::io::{Error, Write, ErrorKind};

const ERROR_MSG_STRING_TOO_LONG: &str = "UTF-8 Type cannot exceed 65,535 bytes";

/// The Encode trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
}

impl Encode for Bits {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write(&[self.0])
    }
}

impl Encode for TwoByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write(&self.0.to_be_bytes())
    }
}

impl Encode for FourByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        writer.write(&self.0.to_be_bytes())
    }
}

impl Encode for UTF8String {
    fn encode<W>(&self, writer: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let data = &self.0;
        let len = data.len();
        if len > i16::max_value() as usize {
            return Err(Error::new(
                ErrorKind::InvalidData,
                ERROR_MSG_STRING_TOO_LONG,
            ));
        }
        writer.write(&(len as u16).to_be_bytes())?;
        writer.write_all(data)?;
        Ok(2 + len)
    }
}

#[cfg(test)]
mod unit_encode {

    use super::*;

    #[test]
    fn encode_bits() {
        let input = Bits(0b00101010);
        let mut result: Vec<u8> = Vec::new();
        let expected = vec![0x2A];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_two_byte_integer() {
        let input = TwoByteInteger(1984u16);
        let mut result: Vec<u8> = Vec::new();
        let expected = vec![0x07, 0xC0];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_four_byte_integer() {
        let input = FourByteInteger(220_000_u32);
        let mut result: Vec<u8> = Vec::new();
        let expected = vec![0x00, 0x03, 0x5B, 0x60];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 4);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }
    
    #[test]
    fn encode_utf8_string_00() {
        let input = UTF8String(Vec::from("A𪛔".as_bytes()));
        let mut result: Vec<u8> = Vec::new();
        let expected = vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 7);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }
    
    #[test]
    fn encode_utf8_string_empty() {
        let input = UTF8String(Vec::new());
        let mut result: Vec<u8> = Vec::new();
        let expected = vec![0x00, 0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }
}
