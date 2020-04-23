use crate::{BinaryData, Error, ReadTwoByteInteger, Result as SageResult};
use std::io::Read;

/// The `Decode` trait is implemented for any type that
/// can be read from a stream.
pub trait Decode: Sized {
    /// Reads the input `Reader` and returns the parsed data.
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self>;
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
    use std::io::Cursor;

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
