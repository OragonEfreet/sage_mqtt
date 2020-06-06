use crate::{codec, Error, Result as SageResult};
use futures::io::{
    AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Error as IOError, ErrorKind,
};
use std::marker::Unpin;

/// Write the given `data` into `writer` according to Binary Data type MQTT5 specifications
/// which consists in a two bytes integer representing the data size in bytes followed with
/// the data as bytes.
/// In case of success returns the written size in bytes.
pub async fn write_binary_data<W: AsyncWrite + Unpin>(
    data: &[u8],
    writer: &mut W,
) -> SageResult<usize> {
    let len = data.len();
    if len > i16::max_value() as usize {
        return Err(IOError::new(ErrorKind::InvalidData, "ERROR_MSG_DATA_TOO_LONG").into());
    }
    writer.write_all(&(len as u16).to_be_bytes()).await?;
    writer.write_all(data).await?;
    Ok(2 + len)
}

/// Read from the given reader for binary dataset according to Binary Data type
/// MQTT5 specifications which consists in an two bytes integer representing
/// the data size in bytes followed with the data as bytes.
/// In case of success, returns a `Vec<u8>`
pub async fn read_binary_data<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Vec<u8>> {
    let mut chunk = reader.take(2);
    let size = codec::read_two_byte_integer(&mut chunk).await? as usize;

    let mut data_buffer = Vec::with_capacity(size);
    if size > 0 {
        let mut chunk = reader.take(size as u64);
        match chunk.read_to_end(&mut data_buffer).await {
            Ok(n) if n == size => Ok(data_buffer),
            _ => Err(Error::MalformedPacket),
        }
    } else {
        Ok(Default::default())
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use async_std::io::Cursor;

    #[async_std::test]
    async fn encode() {
        let input = Vec::from("A𪛔".as_bytes());
        let mut result = Vec::new();
        assert_eq!(write_binary_data(&input, &mut result).await.unwrap(), 7);
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[async_std::test]
    async fn encode_empty() {
        let mut result = Vec::new();
        assert_eq!(
            write_binary_data(&Vec::new(), &mut result).await.unwrap(),
            2
        );
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            read_binary_data(&mut test_stream).await.unwrap(),
            Vec::from("A𪛔".as_bytes())
        );
    }

    #[async_std::test]
    async fn decode_empty() {
        let mut test_stream = Cursor::new([0x00, 0x00]);
        assert_eq!(
            read_binary_data(&mut test_stream).await.unwrap(),
            Vec::new()
        );
    }

    #[async_std::test]
    async fn decode_eof() {
        let mut test_stream = Cursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            read_binary_data(&mut test_stream).await,
            Err(Error::MalformedPacket)
        );
    }
}
