use crate::{
    BinaryData, Bits, Error, FourByteInteger, Result as SageResult, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
use std::io::{Cursor, Error as IOError, ErrorKind, Read};
use unicode_reader::CodePoints;

/// The `Decode` trait is implemented for any type that
/// can be read from a stream.
pub trait Decode : Sized {
    /// Reads the input `Reader` and returns the parsed data.
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl Decode for Bits {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0u8; 1];
        reader.read(&mut buf)?;
        Ok(Bits(buf[0]))
    }
}

impl Decode for TwoByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0x8, 2];
        reader.read(&mut buf)?;
        Ok(TwoByteInteger(((buf[0] as u16) << 8) | buf[1] as u16))
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
    fn decode_twobyte_integer() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(TwoByteInteger::decode(&mut test_stream).unwrap(), TwoByteInteger(1984u16));
    }
}
