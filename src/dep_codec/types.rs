/// Text fields in an MQTT paquet are described in UTF-8. Each string in the
/// stream is prefixed with a two-byte size information with MSB first.
/// Because of that, a string cannot be longer that 65,535 bytes.
/// This type can be converted from and to `String`.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct UTF8String(pub String);

impl From<String> for UTF8String {
    fn from(value: String) -> Self {
        UTF8String(value)
    }
}

impl From<&str> for UTF8String {
    fn from(value: &str) -> Self {
        UTF8String(String::from(value))
    }
}

impl From<UTF8String> for String {
    fn from(value: UTF8String) -> Self {
        value.0
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
/// When an unsigned integer encoded, the smallest
/// representation is used.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct VariableByteInteger(pub u32);

impl From<u32> for VariableByteInteger {
    fn from(value: u32) -> Self {
        VariableByteInteger(value)
    }
}

impl From<VariableByteInteger> for u32 {
    fn from(value: VariableByteInteger) -> Self {
        value.0
    }
}

impl From<VariableByteInteger> for u64 {
    fn from(value: VariableByteInteger) -> Self {
        value.0 as u64
    }
}

/// Binary Data is represented by a Two Byte Integer length which indicates the
/// number of data bytes, followed by that number of bytes. Thus, the length of
/// Binary Data is limited to the range of 0 to 65,535 Bytes.
/// This type can be converted from and to `Vec<u8>`.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BinaryData(pub Vec<u8>);

impl From<Vec<u8>> for BinaryData {
    fn from(data: Vec<u8>) -> Self {
        BinaryData(data)
    }
}

impl From<BinaryData> for Vec<u8> {
    fn from(data: BinaryData) -> Self {
        data.0
    }
}
