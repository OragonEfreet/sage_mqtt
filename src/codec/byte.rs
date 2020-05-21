use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

pub fn write_byte<W: Write>(byte: u8, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[byte])?)
}

pub fn read_bool<R: Read>(reader: &mut R) -> SageResult<bool> {
    let byte = read_byte(reader)?;
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::ProtocolError),
    }
}

pub fn write_bool<W: Write>(data: bool, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[data as u8])?)
}

pub fn read_byte<R: Read>(reader: &mut R) -> SageResult<u8> {
    let mut buf = [0u8; 1];
    if reader.read_exact(&mut buf).is_ok() {
        Ok(buf[0])
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
        assert_eq!(write_byte(0b00101010, &mut result).unwrap(), 1);
        assert_eq!(result, vec![0x2A]);
    }

    #[test]
    fn decode() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        assert_eq!(read_byte(&mut test_stream).unwrap(), 0xAF);
    }

    #[test]
    fn decode_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        assert_matches!(read_byte(&mut test_stream), Err(Error::MalformedPacket));
    }
}
