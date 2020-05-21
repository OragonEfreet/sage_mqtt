use crate::{codec, Error, Result as SageResult};
use std::io::{Cursor, Read, Write};
use unicode_reader::CodePoints;

pub fn write_utf8_string<W: Write>(data: &str, writer: &mut W) -> SageResult<usize> {
    let len = data.len();
    if len > i16::max_value() as usize {
        return Err(Error::MalformedPacket);
    }
    writer.write_all(&(len as u16).to_be_bytes())?;
    writer.write_all(data.as_bytes())?;
    Ok(2 + len)
}

pub fn read_utf8_string<R: Read>(reader: &mut R) -> SageResult<String> {
    let mut chunk = reader.take(2);
    let size = codec::read_two_byte_integer(&mut chunk)?;
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

#[cfg(test)]
mod unit {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode() {
        let mut result = Vec::new();
        assert_eq!(write_utf8_string("A𪛔", &mut result).unwrap(), 7);
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[test]
    fn encode_empty() {
        let mut result = Vec::new();
        assert_eq!(write_utf8_string("", &mut result).unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[test]
    fn decode_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(
            read_utf8_string(&mut test_stream).unwrap(),
            String::default()
        );
    }

    #[test]
    fn decode() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            read_utf8_string(&mut test_stream).unwrap(),
            String::from("A𪛔")
        );
    }

    #[test]
    fn decode_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            read_utf8_string(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
