use crate::{Error, Result as SageResult};
use std::io::{Read, Write};

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

#[cfg(test)]
mod unit_codec {

    use std::io::Cursor;

    use super::*;

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
}
