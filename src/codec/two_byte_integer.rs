use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

pub fn write_two_byte_integer<W: Write>(data: u16, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes())?)
}

pub fn read_two_byte_integer<R: Read>(reader: &mut R) -> SageResult<u16> {
    let mut buf = [0_u8; 2];
    if reader.read_exact(&mut buf).is_ok() {
        Ok(((buf[0] as u16) << 8) | buf[1] as u16)
    } else {
        Err(Error::MalformedPacket)
    }
}

#[cfg(test)]
mod unit {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode() {
        let mut result = Vec::new();
        assert_eq!(write_two_byte_integer(1984u16, &mut result).unwrap(), 2);
        assert_eq!(result, vec![0x07, 0xC0]);
    }

    #[test]
    fn decode() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(read_two_byte_integer(&mut test_stream).unwrap(), 1984u16);
    }

    #[test]
    fn decode_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            read_two_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
