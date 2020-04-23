use crate::{BinaryData, Error, ReadTwoByteInteger, Result as SageResult, UTF8String};
use std::io::{Cursor, Read};
use unicode_reader::CodePoints;

/// The `Decode` trait is implemented for any type that
/// can be read from a stream.
pub trait Decode: Sized {
    /// Reads the input `Reader` and returns the parsed data.
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl Decode for UTF8String {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
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

impl Decode for BinaryData {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size = u16::read_two_byte_integer(&mut chunk)? as usize;

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
