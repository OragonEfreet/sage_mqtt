use crate::{
    BinaryData, Bits, Error, FourByteInteger, Result as SageResult, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
use std::io::{Error as IOError, ErrorKind, Write};

/// The `Encode` trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl Encode for Bits {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&[self.0])?)
    }
}

impl Encode for TwoByteInteger {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
}

impl Encode for FourByteInteger {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.0.to_be_bytes())?)
    }
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

impl Encode for VariableByteInteger {
    fn encode<W>(self, writer: &mut W) -> SageResult<usize>
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
    fn encode_bits() {
        let mut result = Vec::new();
        assert_eq!(Bits(0b00101010).encode(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x2A]);
    }

    #[test]
    fn encode_two_byte_integer() {
        let mut result = Vec::new();
        assert_eq!(TwoByteInteger(1984u16).encode(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x07, 0xC0]);
    }

    #[test]
    fn encode_four_byte_integer() {
        let mut result = Vec::new();
        assert_eq!(FourByteInteger(220_000_u32).encode(&mut result).unwrap(), 4);
        assert_eq!(result, vec![0x00, 0x03, 0x5B, 0x60]);
    }

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
        let mut result = Vec::new();
        assert_eq!(VariableByteInteger(0).encode(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x00]);
    }

    #[test]
    fn encode_variable_byte_integer_one_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(VariableByteInteger(127).encode(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_two_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(VariableByteInteger(128).encode(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_two_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(VariableByteInteger(16_383).encode(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0xFF, 0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_three_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(VariableByteInteger(16_384).encode(&mut result).unwrap(), 3);
        assert_eq!(result, vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_three_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            VariableByteInteger(2_097_151).encode(&mut result).unwrap(),
            3
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_four_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            VariableByteInteger(2_097_152).encode(&mut result).unwrap(),
            4
        );
        assert_eq!(result, vec![0x80, 0x80, 0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_four_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            VariableByteInteger(268_435_455)
                .encode(&mut result)
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0xFF, 0x7F]);
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
