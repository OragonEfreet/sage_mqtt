use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

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
}
