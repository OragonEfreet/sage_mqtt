//! Defines the types used within MQTT streams

/// Bits in a byte are labelled 7 to 0. Bit number 7 is the most significant
/// bit.
#[derive(Debug)]
pub struct Bits(pub u8);

/// Two bytes data values are 16-bits unsigned integer represented in
/// big-endian. That means the most significant byte (MSB) is presented first
/// on the stream.
#[derive(Debug)]
pub struct TwoByteInteger(pub u16);

/// Four bytes data values are 32-bits unsigned integer represented in
/// big-endian. That means the most significant byte (MSB) is presented first
/// on the stream.
#[derive(Debug)]
pub struct FourByteInteger(pub u32);

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
