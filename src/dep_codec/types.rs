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
