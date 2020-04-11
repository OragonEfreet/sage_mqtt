use crate::{
    BinaryData, Bits, Error, FourByteInteger, Result as SageResult, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
use std::io::{Cursor, Read};
use unicode_reader::CodePoints;

/// The `Decode` trait is implemented for any type that
/// can be read from a stream.
pub trait Decode: Sized {
    /// Reads the input `Reader` and returns the parsed data.
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self>;
}

impl Decode for Bits {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        Ok(Bits(buf[0]))
    }
}

impl Decode for TwoByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(TwoByteInteger(((buf[0] as u16) << 8) | buf[1] as u16))
    }
}

impl Decode for FourByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buf = [0_u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(FourByteInteger(
            ((buf[0] as u32) << 24)
                | ((buf[1] as u32) << 16)
                | ((buf[2] as u32) << 8)
                | (buf[3] as u32),
        ))
    }
}

impl Decode for UTF8String {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let size: u16 = TwoByteInteger::decode(reader)?.into();
        let size = size as usize;
        let mut buffer = Vec::with_capacity(size);
        unsafe {
            buffer.set_len(size);
        }
        reader.read_exact(&mut buffer)?;

        let mut codepoints = CodePoints::from(Cursor::new(&buffer));
        if codepoints.all(|x| match x {
            Ok('\u{0}') => false,
            Ok(_) => true,
            _ => false, // Will be an IO Error
        }) {
            Ok(UTF8String(buffer))
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Decode for VariableByteInteger {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut buffer = vec![0u8; 1];

        reader.read_exact(&mut buffer)?;
        let b0 = buffer[0];
        if b0 & 128u8 == 0u8 {
            return Ok(VariableByteInteger::One(b0));
        }

        reader.read_exact(&mut buffer)?;
        let b1 = buffer[0];
        if b1 & 128u8 == 0u8 {
            return Ok(VariableByteInteger::Two(b0, b1));
        }

        reader.read_exact(&mut buffer)?;
        let b2 = buffer[0];
        if b2 & 128u8 == 0u8 {
            return Ok(VariableByteInteger::Three(b0, b1, b2));
        }

        reader.read_exact(&mut buffer)?;
        let b3 = buffer[0];
        Ok(VariableByteInteger::Four(b0, b1, b2, b3))
    }
}

impl Decode for BinaryData {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut chunk = reader.take(2);
        let size: u16 = TwoByteInteger::decode(&mut chunk)?.into();
        let mut data_buffer: Vec<u8> = Vec::with_capacity(size as usize);
        println!("The size is -> {}", size);
        if size > 0 {
            let mut chunk = reader.take(size as u64);
            chunk.read_to_end(&mut data_buffer)?;
        }
        Ok(BinaryData::from(data_buffer))
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
        assert_eq!(
            TwoByteInteger::decode(&mut test_stream).unwrap(),
            TwoByteInteger(1984u16)
        );
    }

    #[test]
    fn decode_fourbyte_integer() {
        let mut test_stream = Cursor::new([0x00, 0x03, 0x5B, 0x60]);
        assert_eq!(
            FourByteInteger::decode(&mut test_stream).unwrap(),
            FourByteInteger(220_000_u32)
        );
    }

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
            UTF8String(Vec::from("A𪛔".as_bytes()))
        );
    }

    #[test]
    fn decode_conformance_mqtt_1_5_4_1() {
        let mut test_stream = Cursor::new([0x00, 0x02, 0xD8, 0x00]);
        assert_matches!(
            UTF8String::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn decode_conformance_mqtt_1_5_4_2() {
        let mut test_stream = Cursor::new([0x00, 0x02, 0x00, 0x00]);
        assert_matches!(
            UTF8String::decode(&mut test_stream),
            Err(Error::MalformedPacket)
        );
    }

    #[test]
    fn decode_variable_byte_integer_one_lower_bound() {
        let mut test_stream = Cursor::new([0x00]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::One(0_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_one_upper_bound() {
        let mut test_stream = Cursor::new([0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::One(127_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Two(128_u8, 01_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_two_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Two(255_u8, 127_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Three(128_u8, 128_u8, 01_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_three_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Three(255_u8, 255_u8, 127_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x80, 0x01]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Four(128_u8, 128_u8, 128_u8, 01_u8)
        );
    }

    #[test]
    fn decode_variable_byte_integer_four_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0xFF, 0x7F]);
        assert_eq!(
            VariableByteInteger::decode(&mut test_stream).unwrap(),
            VariableByteInteger::Four(255_u8, 255_u8, 255_u8, 127_u8)
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
}
