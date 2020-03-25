//! Defines the `Encode` trait and implement it for the MQTT types.

use crate::{
    BinaryData, Bits, Error, FourByteInteger, Result as MyResult, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
use std::io::{Error as IOError, ErrorKind, Write};

// const ERROR_MSG_STRING_TOO_LONG: &str = "UTF-8 Type cannot exceed 65,535 bytes";
// const ERROR_MSG_DATA_TOO_LONG: &str = "Binary streams cannot exceed 65,535 bytes";

/// The Encode trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(&self, writer: &mut W) -> MyResult<usize>;
}

impl Encode for Bits {
    fn encode<W: Write>(&self, writer: &mut W) -> MyResult<usize> {
        Ok(writer.write(&[self.0])?)
    }
}

impl Encode for TwoByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> MyResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
}

impl Encode for FourByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> MyResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
}

impl Encode for UTF8String {
    fn encode<W>(&self, writer: &mut W) -> MyResult<usize>
    where
        W: Write,
    {
        let data = &self.0;

        if let Ok(_) = String::from_utf8(data.to_vec()) {
            let len = data.len();
            if len > i16::max_value() as usize {
                return Err(Error::MalformedPacket);
            }
            writer.write_all(&(len as u16).to_be_bytes())?;
            writer.write_all(data)?;
            Ok(2 + len)
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Encode for VariableByteInteger {
    fn encode<W>(&self, writer: &mut W) -> MyResult<usize>
    where
        W: Write,
    {
        let bytes = match self {
            VariableByteInteger::One(b0) => writer.write(&[*b0])?,
            VariableByteInteger::Two(b1, b0) => writer.write(&[*b1, *b0])?,
            VariableByteInteger::Three(b2, b1, b0) => writer.write(&[*b2, *b1, *b0])?,
            VariableByteInteger::Four(b3, b2, b1, b0) => writer.write(&[*b3, *b2, *b1, *b0])?,
        };
        Ok(bytes)
    }
}

impl Encode for BinaryData {
    fn encode<W>(&self, writer: &mut W) -> MyResult<usize>
    where
        W: Write,
    {
        let data = &self.0;
        let len = data.len();
        if len > i16::max_value() as usize {
            return Err(IOError::new(ErrorKind::InvalidData, "ERROR_MSG_DATA_TOO_LONG").into());
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
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
        let mut result = Vec::new();
        let expected = vec![0x2A];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_two_byte_integer() {
        let input = TwoByteInteger(1984u16);
        let mut result = Vec::new();
        let expected = vec![0x07, 0xC0];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_four_byte_integer() {
        let input = FourByteInteger(220_000_u32);
        let mut result = Vec::new();
        let expected = vec![0x00, 0x03, 0x5B, 0x60];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 4);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_utf8_string() {
        let input = UTF8String(Vec::from("A𪛔".as_bytes()));
        let mut result = Vec::new();
        let expected = vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 7);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_utf8_string_empty() {
        let input = UTF8String(Vec::new());
        let mut result = Vec::new();
        let expected = vec![0x00, 0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    // The character data in a UTF-8 Encoded String MUST be well-formed UTF-8 as
    // defined by the Unicode specification [Unicode] and restated in
    // RFC 3629 [RFC3629]. In particular, the character data MUST NOT include
    // encodings of code points between U+D800 and U+DFFF [MQTT-1.5.4-1]
    #[test]
    fn conformance_mqtt_1_5_4_1() {
        let input = UTF8String(vec![0xD8, 0x00]);
        let mut test_stream = Vec::new();
        assert!(matches!(
            input.encode(&mut test_stream),
            Err(Error::MalformedPacket)
        ));
    }

    #[test]
    fn encode_variable_byte_integer_one_lower_bound() {
        let input = VariableByteInteger::One(0_u8);
        let mut result = Vec::new();
        let expected = vec![0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_one_upper_bound() {
        let input = VariableByteInteger::One(127_u8);
        let mut result = Vec::new();
        let expected = vec![0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_two_lower_bound() {
        let input = VariableByteInteger::Two(128_u8, 01_u8);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_two_upper_bound() {
        let input = VariableByteInteger::Two(255_u8, 127_u8);
        let mut result = Vec::new();
        let expected = vec![0xFF, 0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_three_lower_bound() {
        let input = VariableByteInteger::Three(128_u8, 128_u8, 01_u8);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 3);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_three_upper_bound() {
        let input = VariableByteInteger::Three(255_u8, 255_u8, 127_u8);
        let mut result = Vec::new();
        let expected = vec![0xFF, 0xFF, 0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 3);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_four_lower_bound() {
        let input = VariableByteInteger::Four(128_u8, 128_u8, 128_u8, 01_u8);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x80, 0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 4);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_four_upper_bound() {
        let input = VariableByteInteger::Four(255_u8, 255_u8, 255_u8, 127_u8);
        let mut result = Vec::new();
        let expected = vec![0xFF, 0xFF, 0xFF, 0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 4);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_binarydata() {
        let input = BinaryData(Vec::from("A𪛔".as_bytes()));
        let mut result = Vec::new();
        let expected = vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 7);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_binarydata_empty() {
        let input = BinaryData(Vec::new());
        let mut result = Vec::new();
        let expected = vec![0x00, 0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }
}
