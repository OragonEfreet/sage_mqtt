use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

pub fn write_four_byte_integer<W: Write>(data: u32, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes())?)
}

pub fn read_four_byte_integer<R: Read>(reader: &mut R) -> SageResult<u32> {
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

#[cfg(test)]
mod unit {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode() {
        let mut result = Vec::new();
        assert_eq!(
            write_four_byte_integer(220_000_u32, &mut result).unwrap(),
            4
        );
        assert_eq!(result, vec![0x00, 0x03, 0x5B, 0x60]);
    }

    #[test]
    fn decode() {
        let mut test_stream = Cursor::new([0x00, 0x03, 0x5B, 0x60]);
        assert_eq!(
            read_four_byte_integer(&mut test_stream).unwrap(),
            220_000_u32
        );
    }

    #[test]
    fn decode_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            read_four_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
