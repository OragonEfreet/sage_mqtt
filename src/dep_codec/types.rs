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
