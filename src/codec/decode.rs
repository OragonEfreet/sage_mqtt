use crate::{
    BinaryData, Bits, Error, FourByteInteger, Result as SageResult, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
use std::io::{Cursor, Read};
use unicode_reader::CodePoints;

/// The `Decode` trait is implemented for any type that
/// can be read from a stream.
pub trait Decode: Sized {
    /// Reads the input `Reader` and returns the parsed data.
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self>;
}
impl Decode for Bits {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0u8; 1];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(Bits(buf[0]))
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Decode for TwoByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 2];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(TwoByteInteger(((buf[0] as u16) << 8) | buf[1] as u16))
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Decode for FourByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 4];
        if reader.read_exact(&mut buf).is_ok() {
            Ok(FourByteInteger(
                ((buf[0] as u32) << 24)
                    | ((buf[1] as u32) << 16)
                    | ((buf[2] as u32) << 8)
                    | (buf[3] as u32),
            ))
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Decode for UTF8String {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size: u16 = TwoByteInteger::decode(&mut chunk)?.into();
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
                            Ok(UTF8String(string))
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

impl Decode for VariableByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
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

        Ok(VariableByteInteger(value))
    }
}

impl Decode for BinaryData {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size: u16 = TwoByteInteger::decode(&mut chunk)?.into();
        let size = size as usize;

        let mut data_buffer: Vec<u8> = Vec::with_capacity(size);
        if size > 0 {
            let mut chunk = reader.take(size as u64);
            match chunk.read_to_end(&mut data_buffer) {
                Ok(n) if n == size => Ok(BinaryData::from(data_buffer)),
                _ => Err(Error::MalformedPacket),
            }
        } else {
            Ok(Default::default())
        }
    }
}

#[cfg(test)]
mod unit_decode {

    use super::*;

    #[test]
    fn decode_bits() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        assert_eq!(Bits::decode(&mut test_stream).unwrap(), Bits(0xAF));
    }

    #[test]
    fn decode_bits_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        assert_matches!(Bits::decode(&mut test_stream), Err(Error::MalformedPacket));
    }

    #[test]
    fn decode_twobyte_integer() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(
            TwoByteInteger::decode(&mut test_stream).unwrap(),
            TwoByteInteger(1984u16)
        );
    }

    #[test]
    fn decode_twobyte_integer_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            TwoByteInteger::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn decode_fourbyte_integer() {
        let mut test_stream = Cursor::new([0x00, 0x03, 0x5B, 0x60]);
        assert_eq!(
            FourByteInteger::decode(&mut test_stream).unwrap(),
            FourByteInteger(220_000_u32)
        );
    }

    #[test]
    fn decode_fourbyte_integer_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            FourByteInteger::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn decode_utf8string_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(
            UTF8String::decode(&mut test_stream).unwrap(),
            UTF8String::default()
        );
    }

    #[test]
    fn decode_utf8string() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            UTF8String::decode(&mut test_stream).unwrap(),
            UTF8String::from("A𪛔")
        );
    }

    #[test]
    fn decode_utf8string_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            UTF8String::decode(&mut test_stream),
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
            UTF8String::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    // A UTF-8 Encoded String MUST NOT include an encoding of the null character
    // U+0000.
    #[test]
    fn mqtt_1_5_4_2() {
        let mut test_stream = Cursor::new([0x00, 0x02, 0x00, 0x00]);
        assert_matches!(
            UTF8String::decode(&mut test_stream),
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
            UTF8String::decode(&mut test_stream).unwrap(),
            UTF8String::from("\u{feff}A𪛔")
        );
    }

    #[test]
    fn decode_variable_byte_integer_one_lower_bound() {
        let mut test_stream = Cursor::new([0x00]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(0)
        );
    }

    #[test]
    fn decode_variable_byte_integer_one_upper_bound() {
        let mut test_stream = Cursor::new([0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(127)
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(128)
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(16_383)
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(16_384)
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(2_097_151)
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(2_097_152)
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger(268_435_455)
        );
    }

    #[test]
    fn decode_variable_byte_integer_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        assert_matches!(
            VariableByteInteger::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn decode_binary_data() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            BinaryData::decode(&mut test_stream).unwrap(),
            BinaryData(Vec::from("A𪛔".as_bytes()))
        );
    }

    #[test]
    fn decode_binary_data_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(
            BinaryData::decode(&mut test_stream).unwrap(),
            BinaryData(Default::default())
        );
    }

    #[test]
    fn decode_binary_data_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            BinaryData::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
