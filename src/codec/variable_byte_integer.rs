use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

pub trait WriteVariableByteInteger {
    fn write_variable_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}
impl WriteVariableByteInteger for usize {
    fn write_variable_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        (self as u32).write_variable_byte_integer(writer)
    }
}

impl WriteVariableByteInteger for u32 {
    fn write_variable_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_encoded_bytes = 0;
        let mut x = self;
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

pub trait ReadVariableByteInteger: Sized {
    fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadVariableByteInteger for usize {
    fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        Ok(u32::read_variable_byte_integer(reader)? as usize)
    }
}
impl ReadVariableByteInteger for u64 {
    fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        Ok(u32::read_variable_byte_integer(reader)? as u64)
    }
}
impl ReadVariableByteInteger for u32 {
    fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut multiplier = 1_u32;
        let mut value = 0_u32;

        loop {
            let mut buffer = vec![0u8; 1];
            if reader.read_exact(&mut buffer).is_ok() {
                let encoded_byte = buffer[0];
                value += ((encoded_byte & 127u8) as u32) * multiplier;
                if multiplier > 2_097_152 {
                    return Err(Error::MalformedPacket);
                }
                multiplier *= 128;
                if encoded_byte & 128u8 == 0 {
                    break;
                }
            } else {
                return Err(Error::MalformedPacket);
            }
        }

        Ok(value)
    }
}

#[cfg(test)]
mod unit_codec {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode_variable_byte_integer_one_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(0u32.write_variable_byte_integer(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x00]);
    }

    #[test]
    fn encode_variable_byte_integer_one_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(127u32.write_variable_byte_integer(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_two_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(128u32.write_variable_byte_integer(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_two_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            16_383u32.write_variable_byte_integer(&mut result).unwrap(),
            2
        );
        assert_eq!(result, vec![0xFF, 0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_three_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            16_384u32.write_variable_byte_integer(&mut result).unwrap(),
            3
        );
        assert_eq!(result, vec![0x80, 0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_three_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            2_097_151u32
                .write_variable_byte_integer(&mut result)
                .unwrap(),
            3
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn encode_variable_byte_integer_four_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            2_097_152u32
                .write_variable_byte_integer(&mut result)
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0x80, 0x80, 0x80, 0x01]);
    }

    #[test]
    fn encode_variable_byte_integer_four_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            268_435_455u32
                .write_variable_byte_integer(&mut result)
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0xFF, 0x7F]);
    }

    #[test]
    fn decode_variable_byte_integer_one_lower_bound() {
        let mut test_stream = Cursor::new([0x00]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            0u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_one_upper_bound() {
        let mut test_stream = Cursor::new([0x7F]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            127u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x01]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            128u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0x7F]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            16_383u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x01]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            16_384u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0x7F]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            2_097_151u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x80, 0x01]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            2_097_152u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0xFF, 0x7F]);
        assert_eq!(
            u32::read_variable_byte_integer(&mut test_stream).unwrap(),
            268_435_455u32
        );
    }

    #[test]
    fn decode_variable_byte_integer_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        assert_matches!(
            u32::read_variable_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
