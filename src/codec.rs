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

// pub trait WriteFourByteInteger {
//     fn write_four_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize>;
// }

// pub trait ReadFourByteInteger : Sized {
//     fn read_four_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self>;
// }

// pub trait WriteVariableByteInteger {
//     fn write_variable_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize>;
// }

// pub trait ReadVariableByteInteger : Sized {
//     fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self>;
// }

// pub trait WriteUTF8String {
//     fn write_utf8_string<W: Write>(self, writer: &mut W) -> SageResult<usize>;
// }

// pub trait ReadUTF8String : Sized {
//     fn read_utf8_string<R: Read>(reader: &mut R) -> SageResult<Self>;
// }

// pub trait WriteBinaryData {
//     fn write_binary_data<W: Write>(self, writer: &mut W) -> SageResult<usize>;
// }

// pub trait ReadBinaryData : Sized {
//     fn read_binary_data<R: Read>(reader: &mut R) -> SageResult<Self>;
// }

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
    fn write_byte() {
        let mut result = Vec::new();
        assert_eq!(0b00101010.write_byte(&mut result).unwrap(), 1);
        assert_eq!(result, vec![0x2A]);
    }

    #[test]
    fn read_byte() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        assert_eq!(u8::read_byte(&mut test_stream).unwrap(), 0xAF);
    }

    #[test]
    fn read_byte_eof() {
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
    fn read_two_byte_integer() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(
            u16::read_two_byte_integer(&mut test_stream).unwrap(),
            1984u16
        );
    }

    #[test]
    fn read_two_byte_integer_eof() {
        let mut test_stream = Cursor::new([0x07]);
        assert_matches!(
            u16::read_two_byte_integer(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }
}
