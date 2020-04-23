use crate::{Error, Result as SageResult};
use std::io::{Cursor, Error as IOError, ErrorKind, Read, Write};
use unicode_reader::CodePoints;

pub trait WriteByte {
    fn write_byte<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteByte for u8 {
    fn write_byte<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&[self])?)
    }
}

impl WriteByte for bool {
    fn write_byte<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&[self as u8])?)
    }
}

pub trait ReadByte: Sized {
    fn read_byte<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadByte for u8 {
    fn read_byte<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0u8; 1];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(buf[0])
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

pub trait WriteTwoByteInteger {
    fn write_two_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteTwoByteInteger for u16 {
    fn write_two_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.to_be_bytes())?)
    }
}

pub trait ReadTwoByteInteger: Sized {
    fn read_two_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadTwoByteInteger for u16 {
    fn read_two_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 2];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(((buf[0] as u16) << 8) | buf[1] as u16)
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

pub trait WriteFourByteInteger {
    fn write_four_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteFourByteInteger for u32 {
    fn write_four_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Ok(writer.write(&self.to_be_bytes())?)
    }
}

pub trait ReadFourByteInteger: Sized {
    fn read_four_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadFourByteInteger for u32 {
    fn read_four_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 4];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(((buf[0] as u32) << 24)
                | ((buf[1] as u32) << 16)
                | ((buf[2] as u32) << 8)
                | (buf[3] as u32))
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

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

pub trait WriteUTF8String {
    fn write_utf8_string<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteUTF8String for &str {
    fn write_utf8_string<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let len = self.len();
        if len > i16::max_value() as usize {
            return Err(Error::MalformedPacket);
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(self.as_bytes())?;
        Ok(2 + len)
    }
}

pub trait ReadUTF8String: Sized {
    fn read_utf8_string<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadUTF8String for String {
    fn read_utf8_string<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size = u16::read_two_byte_integer(&mut chunk)?;
        let size = size as usize;

        let mut data_buffer: Vec<u8> = Vec::with_capacity(size);
        if size > 0 {
            let mut chunk = reader.take(size as u64);
            match chunk.read_to_end(&mut data_buffer) {
                Ok(n) if n == size => {
                    let mut codepoints = CodePoints::from(Cursor::new(&data_buffer));
                    if codepoints.all(|x| match x {
                        Ok('\u{0}') => false,
                        Ok(_) => true,
                        _ => false, // Will be an IO Error
                    }) {
                        if let Ok(string) = String::from_utf8(data_buffer) {
                            Ok(string)
                        } else {
                            Err(Error::MalformedPacket)
                        }
                    } else {
                        Err(Error::MalformedPacket)
                    }
                }
                _ => Err(Error::MalformedPacket),
            }
        } else {
            Ok(Default::default())
        }
    }
}

pub trait WriteBinaryData {
    fn write_binary_data<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}

impl WriteBinaryData for Vec<u8> {
    fn write_binary_data<W>(self, writer: &mut W) -> SageResult<usize>
    where
        W: Write,
    {
        let len = self.len();
        if len > i16::max_value() as usize {
            return Err(IOError::new(ErrorKind::InvalidData, "ERROR_MSG_DATA_TOO_LONG").into());
        }
        writer.write_all(&(len as u16).to_be_bytes())?;
        writer.write_all(&self)?;
        Ok(2 + len)
    }
}

pub trait ReadBinaryData: Sized {
    fn read_binary_data<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl ReadBinaryData for Vec<u8> {
    fn read_binary_data<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size = u16::read_two_byte_integer(&mut chunk)? as usize;

        let mut data_buffer: Vec<u8> = Vec::with_capacity(size);
        if size > 0 {
            let mut chunk = reader.take(size as u64);
            match chunk.read_to_end(&mut data_buffer) {
                Ok(n) if n == size => Ok(data_buffer),
                _ => Err(Error::MalformedPacket),
            }
        } else {
            Ok(Default::default())
        }
    }
}

impl ReadByte for bool {
    fn read_byte<R: Read>(reader: &mut R) -> SageResult<Self> {
        let byte = u8::read_byte(reader)?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::ProtocolError),
        }
    }
}

#[cfg(test)]
mod unit_codec {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode_byte() {
        let mut result = Vec::new();
        assert_eq!(0b00101010.write_byte(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x2A]);
    }

    #[test]
    fn decode_byte() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        assert_eq!(u8::read_byte(&mut test_stream).unwrap(), 0xAF);
    }

    #[test]
    fn decode_byte_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        assert_matches!(u8::read_byte(&mut test_stream), Err(Error::MalformedPacket));
    }

    #[test]
    fn encode_two_byte_integer() {
        let mut result = Vec::new();
        assert_eq!(1984u16.write_two_byte_integer(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x07, 0xC0]);
    }

    #[test]
    fn decode_two_byte_integer() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(
            u16::read_two_byte_integer(&mut test_stream).unwrap(),
            1984u16
        );
    }

    #[test]
    fn decode_two_byte_integer_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            u16::read_two_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn encode_four_byte_integer() {
        let mut result = Vec::new();
        assert_eq!(220_000_u32.write_four_byte_integer(&mut result).unwrap(), 4);
        assert_eq!(result, vec![0x00, 0x03, 0x5B, 0x60]);
    }

    #[test]
    fn decode_four_byte_integer() {
        let mut test_stream = Cursor::new([0x00, 0x03, 0x5B, 0x60]);
        assert_eq!(
            u32::read_four_byte_integer(&mut test_stream).unwrap(),
            220_000_u32
        );
    }

    #[test]
    fn decode_four_byte_integer_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            u32::read_four_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    // The encoded value MUST use the minimum number of bytes necessary to
    // represent the value
    // Note: This test considers the fact that if VALUE_L and VALUE_R are
    // both encoded into N bytes, then all values between VALUE_L and VALUE_R
    // are encoded into N bytes as well. Meaning: we only check bounds.
    #[test]
    fn mqtt_1_5_5_1() {
        let bounds = [
            [0u32, 12_u32],
            [128_u32, 16_383_u32],
            [16_384_u32, 2_097_151_u32],
            [2_097_152_u32, 268_435_455_u32],
        ];

        let mut result = Vec::new();

        let mut expected_buffer_size = 1;

        for bound in &bounds {
            for i in bound {
                let n_bytes = i.write_variable_byte_integer(&mut result).unwrap();
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

    #[test]
    fn encode_utf8string() {
        let mut result = Vec::new();
        assert_eq!("A𪛔".write_utf8_string(&mut result).unwrap(), 7);
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_utf8string_empty() {
        let mut result = Vec::new();
        assert_eq!("".write_utf8_string(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[test]
    fn decode_utf8string_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(
            String::read_utf8_string(&mut test_stream).unwrap(),
            String::default()
        );
    }

    #[test]
    fn decode_utf8string() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            String::read_utf8_string(&mut test_stream).unwrap(),
            String::from("A𪛔")
        );
    }

    #[test]
    fn decode_utf8string_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            String::read_utf8_string(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    // The character data in a UTF-8 Encoded String MUST be well-formed UTF-8 as
    // defined by the Unicode specification [Unicode] and restated in RFC 3629
    // [RFC3629]. In particular, the character data MUST NOT include encodings
    // of code points between U+D800 and U+DFFF
    #[test]
    fn mqtt_1_5_4_1() {
        let mut test_stream = Cursor::new([0x00, 0x02, 0xD8, 0x00]);
        assert_matches!(
            String::read_utf8_string(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    // A UTF-8 Encoded String MUST NOT include an encoding of the null character
    // U+0000.
    #[test]
    fn mqtt_1_5_4_2() {
        let mut test_stream = Cursor::new([0x00, 0x02, 0x00, 0x00]);
        assert_matches!(
            String::read_utf8_string(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    // A UTF-8 encoded sequence 0xEF 0xBB 0xBF is always interpreted as U+FEFF
    // ("ZERO WIDTH NO-BREAK SPACE") wherever it appears in a string and MUST
    // NOT be skipped over or stripped off by a packet receiver
    #[test]
    fn mqtt_1_5_4_3() {
        let mut test_stream =
            Cursor::new([0x00, 0x08, 0xEF, 0xBB, 0xBF, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            String::read_utf8_string(&mut test_stream).unwrap(),
            String::from("\u{feff}A𪛔")
        );
    }

    #[test]
    fn encode_binarydata() {
        let mut result = Vec::new();
        assert_eq!(
            Vec::from("A𪛔".as_bytes())
                .write_binary_data(&mut result)
                .unwrap(),
            7
        );
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_binarydata_empty() {
        let mut result = Vec::new();
        assert_eq!(Vec::new().write_binary_data(&mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[test]
    fn decode_binary_data() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            Vec::read_binary_data(&mut test_stream).unwrap(),
            Vec::from("A𪛔".as_bytes())
        );
    }

    #[test]
    fn decode_binary_data_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(Vec::read_binary_data(&mut test_stream).unwrap(), Vec::new());
    }

    #[test]
    fn decode_binary_data_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            Vec::read_binary_data(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
