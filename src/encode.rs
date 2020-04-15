use crate::{
    BinaryData, Bits, Byte, Error, FourByteInteger, Result as SageResult, TwoByteInteger,
    UTF8String, VariableByteInteger,
};
use std::io::{Error as IOError, ErrorKind, Write};

/// The `Encode` trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize>;
}

impl Encode for Byte {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&[self.0])?)
    }
}

impl Encode for Bits {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&[self.0])?)
    }
}

impl Encode for TwoByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
}

impl Encode for FourByteInteger {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
}

impl Encode for UTF8String {
    fn encode<W>(&self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let data = &self.0;
        let len = data.len();
        if len > i16::max_value() as usize {
            return Err(Error::MalformedPacket);
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(data.as_bytes())?;
        Ok(2 + len)
    }
}

impl Encode for VariableByteInteger {
    fn encode<W>(&self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let mut n_encoded_bytes = 0;
        let mut x = self.0;
        loop {
            let mut encoded_byte = (x % 128) as u8;
            x /= 128;
            if x > 0 {
                encoded_byte |= 128u8;
            }
            n_encoded_bytes += writer.write(&[encoded_byte])?;
            if x == 0 {
                break;
            }
        }
        Ok(n_encoded_bytes)
    }
}

impl Encode for BinaryData {
    fn encode<W>(&self, writer: &mut W) -> SageResult<usize>
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
    fn encode_byte() {
        let input = Byte(0b00101010);
        let mut result = Vec::new();
        let expected = vec![0x2A];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

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
    fn encode_utf8string() {
        let input = UTF8String::from("A𪛔");
        let mut result = Vec::new();
        let expected = vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 7);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_utf8string_empty() {
        let input = UTF8String::default();
        let mut result = Vec::new();
        let expected = vec![0x00, 0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    // The encoded value MUST use the minimum number of bytes necessary to
    // represent the value
    // Note: This test considers the fact that if VALUE_L and VALUE_R are
    // both encoded into N bytes, then all values between VALUE_L and VALUE_R
    // are encoded into N bytes as well. Meaning: we only check bounds.
    #[test]
    fn mqtt_1_5_5_1() {
        let bounds = [
            [0, 12],
            [128, 16_383],
            [16_384, 2_097_151],
            [2_097_152, 268_435_455],
        ];

        let mut result = Vec::new();

        let mut expected_buffer_size = 1;

        for bound in &bounds {
            for i in bound {
                let input = VariableByteInteger::from(*i);
                let n_bytes = input.encode(&mut result).unwrap();
                assert_eq!(
                    n_bytes, expected_buffer_size,
                    "Variable Byte Integer '{}' should be encoded to '{}' bytes. Used '{}' instead",
                    i, expected_buffer_size, n_bytes
                );
                result.clear();
            }

            expected_buffer_size += 1;
        }
    }

    #[test]
    fn encode_variable_byte_integer_one_lower_bound() {
        let input = VariableByteInteger(0);
        let mut result = Vec::new();
        let expected = vec![0x00];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_one_upper_bound() {
        let input = VariableByteInteger(127);
        let mut result = Vec::new();
        let expected = vec![0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 1);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_two_lower_bound() {
        let input = VariableByteInteger(128);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_two_upper_bound() {
        let input = VariableByteInteger(16_383);
        let mut result = Vec::new();
        let expected = vec![0xFF, 0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 2);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_three_lower_bound() {
        let input = VariableByteInteger(16_384);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 3);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_three_upper_bound() {
        let input = VariableByteInteger(2_097_151);
        let mut result = Vec::new();
        let expected = vec![0xFF, 0xFF, 0x7F];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 3);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_four_lower_bound() {
        let input = VariableByteInteger(2_097_152);
        let mut result = Vec::new();
        let expected = vec![0x80, 0x80, 0x80, 0x01];
        let bytes = input.encode(&mut result).unwrap();
        assert_eq!(bytes, 4);
        assert_eq!(result, expected, "Encoding {:?} failed", input);
    }

    #[test]
    fn encode_variable_byte_integer_four_upper_bound() {
        let input = VariableByteInteger(268_435_455);
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
