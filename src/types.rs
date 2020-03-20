//! Defines the types used within MQTT streams

/// Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant
/// bit.
#[derive(Debug, PartialEq, Eq)]
pub struct Bits(pub u8);

impl From<u8> for Bits {
    fn from(value: u8) -> Self {
        Bits(value)
    }
}

impl From<Bits> for u8 {
    fn from(value: Bits) -> Self {
        value.0
    }
}

/// Two bytes data values are 16-bits unsigned integer represented in
/// big-endian. That means the most significant byte (MSB) is presented first
/// on the stream.
#[derive(Debug, PartialEq, Eq)]
pub struct TwoByteInteger(pub u16);

impl From<u8> for TwoByteInteger {
    fn from(value: u8) -> Self {
        TwoByteInteger(value as u16)
    }
}

impl From<u16> for TwoByteInteger {
    fn from(value: u16) -> Self {
        TwoByteInteger(value)
    }
}

impl From<TwoByteInteger> for u16 {
    fn from(value: TwoByteInteger) -> Self {
        value.0
    }
}

impl From<TwoByteInteger> for u32 {
    fn from(value: TwoByteInteger) -> Self {
        value.0 as u32
    }
}

impl From<TwoByteInteger> for u64 {
    fn from(value: TwoByteInteger) -> Self {
        value.0 as u64
    }
}

/// Four bytes data values are 32-bits unsigned integer represented in
/// big-endian. That means the most significant byte (MSB) is presented first
/// on the stream.
#[derive(Debug, PartialEq, Eq)]
pub struct FourByteInteger(pub u32);

impl From<u8> for FourByteInteger {
    fn from(value: u8) -> Self {
        FourByteInteger(value as u32)
    }
}

impl From<u16> for FourByteInteger {
    fn from(value: u16) -> Self {
        FourByteInteger(value as u32)
    }
}

impl From<u32> for FourByteInteger {
    fn from(value: u32) -> Self {
        FourByteInteger(value)
    }
}

impl From<FourByteInteger> for u32 {
    fn from(value: FourByteInteger) -> Self {
        value.0 as u32
    }
}

impl From<FourByteInteger> for u64 {
    fn from(value: FourByteInteger) -> Self {
        value.0 as u64
    }
}

/// Text fields in an MQTT paquet are described in UTF-8. Each string in the
/// stream is prefixed with a two-byte size information with MSB first.
/// Because of that, a string cannot be longer that 65,535 bytes.
#[derive(Debug)]
pub struct UTF8String(pub Vec<u8>);

/// The Variable Byte Integer is encoded using an encoding scheme which uses
/// a single byte for values up to 127. Larger values are handled as follows.
/// The least significant seven bits of each byte encode the data,
/// and the most significant bit is used to indicate whether there are bytes
/// following in the representation. Thus, each byte encodes 128 values and a
/// "continuation bit". The maximum number of bytes in the Variable Byte
/// Integer field is four.
#[derive(Debug)]
pub enum VariableByteInteger {
    /// From `0` (`0x00`) to `127` (`0x7F`)
    One(u8),
    /// From `128` (`0x80, `0x01`) to `16,383` (`0xFF`, `0x7F`)
    Two(u8, u8),
    /// From `16,384` (`0x80, `0x80, `0x01`) to `2,097,151` (`0xFF`, `0xFF`, `0x7F`)
    Three(u8, u8, u8),
    /// From `2,097,151` (`0x80, `0x80, `0x80, `0x01`) to `268,435,455` (`0xFF`, `0xFF`, `0xFF`, `0x7F`)
    Four(u8, u8, u8, u8),
}

/// Binary Data is represented by a Two Byte Integer length which indicates the
/// number of data bytes, followed by that number of bytes. Thus, the length of
/// Binary Data is limited to the range of 0 to 65,535 Bytes.
#[derive(Debug)]
pub struct BinaryData(pub Vec<u8>);

#[cfg(test)]
mod unit_types {

    use super::*;

    #[test]
    fn test_convert_bits_to_u8() {
        let input = Bits(42_u8);
        let expected = 42_u8;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u8_to_bits() {
        let input = 42_u8;
        let expected = Bits(42_u8);
        let result: Bits = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u8_to_twobyteinteger() {
        let input = 42_u8;
        let expected = TwoByteInteger(42_u16);
        let result: TwoByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u16_to_twobyteinteger() {
        let input = 1984u16;
        let expected = TwoByteInteger(1984u16);
        let result: TwoByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_twobyteinteger_to_u16() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_twobyteinteger_to_u32() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_twobyteinteger_to_u64() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u64;
        let result: u64 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u8_to_fourbyteinteger() {
        let input = 42_u8;
        let expected = FourByteInteger(42_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u16_to_fourbyteinteger() {
        let input = 1984u16;
        let expected = FourByteInteger(1984_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_u32_to_fourbyteinteger() {
        let input = 3735928559_u32;
        let expected = FourByteInteger(3735928559_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_fourbyteinteger_to_u32() {
        let input = FourByteInteger(3735928559_u32);
        let expected = 3735928559_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_convert_fourbyteinteger_to_u64() {
        let input = FourByteInteger(3735928559_u32);
        let expected = 3735928559_u64;
        let result: u64 = input.into();
        assert_eq!(expected, result);
    }
}
