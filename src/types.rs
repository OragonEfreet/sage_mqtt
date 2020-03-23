//! Defines the types used within MQTT streams
use std::convert::TryFrom;

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
#[derive(Debug, PartialEq, Eq)]
pub struct UTF8String(pub Vec<u8>);

impl From<String> for UTF8String {
    fn from(value: String) -> Self {
        UTF8String(value.into_bytes())
    }
}

impl From<UTF8String> for String {
    fn from(value: UTF8String) -> Self {
        if let Ok(result) = String::from_utf8(value.0) {
            result
        } else {
            String::new()
        }
    }
}

/// The Variable Byte Integer is encoded using an encoding scheme which uses
/// a single byte for values up to 127. Larger values are handled as follows.
/// The least significant seven bits of each byte encode the data,
/// and the most significant bit is used to indicate whether there are bytes
/// following in the representation. Thus, each byte encodes 128 values and a
/// "continuation bit". The maximum number of bytes in the Variable Byte
/// Integer field is four.
///
/// When an unsigned integer is converted to `VariableByteInteger`, the smallest
/// representation is used.
/// Upon converting from and to `u8`, `u16` and `u32`, the overflow will result
/// in the maximum value of the target type.
#[derive(Debug, PartialEq, Eq)]
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

impl From<u8> for VariableByteInteger {
    fn from(value: u8) -> Self {
        VariableByteInteger::One(value)
    }
}

impl From<u16> for VariableByteInteger {
    fn from(value: u16) -> Self {
        (value as u32).into()
    }
}

impl From<u32> for VariableByteInteger {
    fn from(value: u32) -> Self {
        let mut bytes: Option<VariableByteInteger> = None;
        let mut x = value;
        loop {
            let mut byte = (x % 128) as u8;
            x /= 128;

            if x > 0 {
                byte |= 128;
            }

            bytes = if let Some(bytes) = bytes {
                match bytes {
                    VariableByteInteger::One(b0) => Some(VariableByteInteger::Two(b0, byte)),
                    VariableByteInteger::Two(b0, b1) => {
                        Some(VariableByteInteger::Three(b0, b1, byte))
                    }
                    VariableByteInteger::Three(b0, b1, b2) => {
                        Some(VariableByteInteger::Four(b0, b1, b2, byte))
                    }
                    _ => Some(VariableByteInteger::Four(0xFF, 0xFF, 0xFF, 0x7F)),
                }
            } else {
                Some(VariableByteInteger::One(byte))
            };

            if x == 0 {
                break;
            }
        }

        bytes.unwrap()
    }
}

impl From<VariableByteInteger> for u8 {
    fn from(vbi: VariableByteInteger) -> Self {
        let a: u32 = vbi.into();
        if let Ok(value) = u8::try_from(a) {
            value
        } else {
            std::u8::MAX
        }
    }
}

impl From<VariableByteInteger> for u16 {
    fn from(vbi: VariableByteInteger) -> Self {
        let a: u32 = vbi.into();
        if let Ok(value) = u16::try_from(a) {
            value
        } else {
            std::u16::MAX
        }
    }
}

impl From<VariableByteInteger> for u32 {
    fn from(vbi: VariableByteInteger) -> Self {
        match vbi {
            VariableByteInteger::One(byte) => byte as u32,
            VariableByteInteger::Two(b0, b1) => (b1 as u32 * 128_u32) + (b0 & 127_u8) as u32,
            VariableByteInteger::Three(b0, b1, b2) => {
                (b2 as u32 * 16_384_u32) + ((b1 & 127_u8) as u32 * 128_u32) + (b0 & 127_u8) as u32
            }
            VariableByteInteger::Four(b0, b1, b2, b3) => {
                (b3 as u32 * 2_097_152_u32)
                    + ((b2 & 127_u8) as u32 * 16_384_u32)
                    + ((b1 & 127_u8) as u32 * 128_u32)
                    + (b0 & 127_u8) as u32
            }
        }
    }
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
    fn convert_bits_to_u8() {
        let input = Bits(42_u8);
        let expected = 42_u8;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u8_to_bits() {
        let input = 42_u8;
        let expected = Bits(42_u8);
        let result: Bits = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u8_to_twobyteinteger() {
        let input = 42_u8;
        let expected = TwoByteInteger(42_u16);
        let result: TwoByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u16_to_twobyteinteger() {
        let input = 1984u16;
        let expected = TwoByteInteger(1984u16);
        let result: TwoByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_twobyteinteger_to_u16() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_twobyteinteger_to_u32() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_twobyteinteger_to_u64() {
        let input = TwoByteInteger(1984u16);
        let expected = 1984u64;
        let result: u64 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u8_to_fourbyteinteger() {
        let input = 42_u8;
        let expected = FourByteInteger(42_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u16_to_fourbyteinteger() {
        let input = 1984u16;
        let expected = FourByteInteger(1984_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u32_to_fourbyteinteger() {
        let input = 3735928559_u32;
        let expected = FourByteInteger(3735928559_u32);
        let result: FourByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_fourbyteinteger_to_u32() {
        let input = FourByteInteger(3735928559_u32);
        let expected = 3735928559_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_fourbyteinteger_to_u64() {
        let input = FourByteInteger(3735928559_u32);
        let expected = 3735928559_u64;
        let result: u64 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_string_to_utf8string() {
        let input = String::from("A𪛔");
        let expected = UTF8String(vec![0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        let result: UTF8String = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_empty_string_to_utf8string() {
        let input = String::from("");
        let expected = UTF8String(Vec::new());
        let result: UTF8String = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_utf8string_to_string() {
        let input = UTF8String(vec![0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        let expected = String::from("A𪛔");
        let result: String = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_utf8string_to_empty_string() {
        let input = UTF8String(Vec::new());
        let expected = String::from("");
        let result: String = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_invalid_utf8string_to_string() {
        let input = UTF8String(vec![0x41, 0xF0, 0xC3, 0x28, 0xAA, 0x9B, 0x94]);
        let expected = String::from("");
        let result: String = input.into();
        assert_eq!(expected, result);
    }
    #[test]
    fn convert_u8_to_variablebyteinteger() {
        let input = 42_u8;
        let expected = VariableByteInteger::One(0x2A);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u16_to_variablebyteinteger_one() {
        let input = 42_u16;
        let expected = VariableByteInteger::One(0x2A);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u16_to_variablebyteinteger_two() {
        let input = 1984_u16;
        let expected = VariableByteInteger::Two(0xC0, 0x0F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u32_to_variablebyteinteger_one() {
        let input = 42_u32;
        let expected = VariableByteInteger::One(0x2A);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u32_to_variablebyteinteger_two() {
        let input = 1984_u32;
        let expected = VariableByteInteger::Two(0xC0, 0x0F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u32_to_variablebyteinteger_three() {
        let input = 51966_u32;
        let expected = VariableByteInteger::Three(0xFE, 0x95, 0x03);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_u32_to_variablebyteinteger_four() {
        let input = 16_435_934_u32;
        let expected = VariableByteInteger::Four(0xDE, 0x95, 0xEB, 0x07);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_0_u32_to_variablebyte_integer() {
        let input = 0_u32;
        let expected = VariableByteInteger::One(0x00);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_127_u32_to_variablebyte_integer() {
        let input = 127_u32;
        let expected = VariableByteInteger::One(0x7F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_128_u32_to_variablebyte_integer() {
        let input = 128_u32;
        let expected = VariableByteInteger::Two(0x80, 0x01);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_16_383_u32_to_variablebyte_integer() {
        let input = 16_383_u32;
        let expected = VariableByteInteger::Two(0xFF, 0x7F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_16_384_u32_to_variablebyte_integer() {
        let input = 16_384_u32;
        let expected = VariableByteInteger::Three(0x80, 0x80, 0x01);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_2_097_151_u32_to_variablebyte_integer() {
        let input = 2_097_151_u32;
        let expected = VariableByteInteger::Three(0xFF, 0xFF, 0x7F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_2_097_152_u32_to_variablebyte_integer() {
        let input = 2_097_152_u32;
        let expected = VariableByteInteger::Four(0x80, 0x80, 0x80, 0x01);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_268_435_455_u32_to_variablebyte_integer() {
        let input = 268_435_455_u32;
        let expected = VariableByteInteger::Four(0xFF, 0xFF, 0xFF, 0x7F);
        let result: VariableByteInteger = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_127_to_u8() {
        let input = VariableByteInteger::One(0x7F);
        let expected = 127_u8;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_127_to_u16() {
        let input = VariableByteInteger::One(0x7F);
        let expected = 127_u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_127_to_u32() {
        let input = VariableByteInteger::One(0x7F);
        let expected = 127_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_128_to_u8() {
        let input = VariableByteInteger::Two(0x80, 0x01);
        let expected = 128_u8;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_128_to_u16() {
        let input = VariableByteInteger::Two(0x80, 0x01);
        let expected = 128_u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_128_to_u32() {
        let input = VariableByteInteger::Two(0x80, 0x01);
        let expected = 128_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_383_to_u8() {
        let input = VariableByteInteger::Two(0xFF, 0x7F);
        let expected = std::u8::MAX;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_383_to_u16() {
        let input = VariableByteInteger::Two(0xFF, 0x7F);
        let expected = 16_383_u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_383_to_u32() {
        let input = VariableByteInteger::Two(0xFF, 0x7F);
        let expected = 16_383_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_384_to_u8() {
        let input = VariableByteInteger::Three(0x80, 0x80, 0x01);
        let expected = std::u8::MAX;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_384_to_u16() {
        let input = VariableByteInteger::Three(0x80, 0x80, 0x01);
        let expected = 16_384_u16;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_16_384_to_u32() {
        let input = VariableByteInteger::Three(0x80, 0x80, 0x01);
        let expected = 16_384_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_151_to_u8() {
        let input = VariableByteInteger::Three(0xFF, 0xFF, 0x7F);
        let expected = std::u8::MAX;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_151_to_u16() {
        let input = VariableByteInteger::Three(0xFF, 0xFF, 0x7F);
        let expected = std::u16::MAX;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_151_to_u32() {
        let input = VariableByteInteger::Three(0xFF, 0xFF, 0x7F);
        let expected = 2_097_151_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_152_to_u8() {
        let input = VariableByteInteger::Four(0x80, 0x80, 0x80, 0x01);
        let expected = std::u8::MAX;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_152_to_u16() {
        let input = VariableByteInteger::Four(0x80, 0x80, 0x80, 0x01);
        let expected = std::u16::MAX;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_2_097_152_to_u32() {
        let input = VariableByteInteger::Four(0x80, 0x80, 0x80, 0x01);
        let expected = 2_097_152_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_268_435_455_to_u8() {
        let input = VariableByteInteger::Four(0xFF, 0xFF, 0xFF, 0x7F);
        let expected = std::u8::MAX;
        let result: u8 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_268_435_455_to_u16() {
        let input = VariableByteInteger::Four(0xFF, 0xFF, 0xFF, 0x7F);
        let expected = std::u16::MAX;
        let result: u16 = input.into();
        assert_eq!(expected, result);
    }

    #[test]
    fn convert_variablebyteinteger_268_435_455_to_u32() {
        let input = VariableByteInteger::Four(0xFF, 0xFF, 0xFF, 0x7F);
        let expected = 268_435_455_u32;
        let result: u32 = input.into();
        assert_eq!(expected, result);
    }
}
