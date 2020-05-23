use crate::{codec, Error, Result as SageResult};
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::io::Cursor;
use std::marker::Unpin;
use unicode_reader::CodePoints;

/// Write the given string into `writer` according to UTF8 String type MQTT5 specifications
/// which consists in a two bytes integer representing the string in bytes followed with
/// the string as bytes.
/// In case of success returns the written size in bytes.
pub async fn write_utf8_string<W: AsyncWrite + Unpin>(
    data: &str,
    writer: &mut W,
) -> SageResult<usize> {
    let len = data.len();
    if len > i16::max_value() as usize {
        return Err(Error::MalformedPacket);
    }
    writer.write_all(&(len as u16).to_be_bytes()).await?;
    writer.write_all(data.as_bytes()).await?;
    Ok(2 + len)
}

/// Read from the given reader for binary dataset according to Binary Data type
/// MQTT5 specifications which consists in an two bytes integer representing
/// the data size in bytes followed with the data as bytes.
/// In case of success, returns a `Vec<u8>`
pub async fn read_utf8_string<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<String> {
    let mut chunk = reader.take(2);
    let size = codec::read_two_byte_integer(&mut chunk).await?;
    let size = size as usize;

    let mut data_buffer: Vec<u8> = Vec::with_capacity(size);
    if size > 0 {
        let mut chunk = reader.take(size as u64);
        match chunk.read_to_end(&mut data_buffer).await {
            Ok(n) if n == size => {
                let mut codepoints = CodePoints::from(Cursor::new(&data_buffer));
                if codepoints.all(|x| match x {
                    Ok('\u{0}') => false,
                    Ok(_) => true,
                    _ => false, // Will be an IO Error
                }) {
                    if let Ok(string) = String::from_utf8(data_buffer) {
                        Ok(string)
                    } else {
                        Err(Error::MalformedPacket)
                    }
                } else {
                    Err(Error::MalformedPacket)
                }
            }
            _ => Err(Error::MalformedPacket),
        }
    } else {
        Ok(Default::default())
    }
}

#[cfg(test)]
mod unit {

    use futures::io::Cursor as AsyncCursor;

    use super::*;

    #[async_std::test]
    async fn encode() {
        let mut result = Vec::new();
        assert_eq!(write_utf8_string("A𪛔", &mut result).await.unwrap(), 7);
        assert_eq!(result, vec![0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
    }

    #[async_std::test]
    async fn encode_empty() {
        let mut result = Vec::new();
        assert_eq!(write_utf8_string("", &mut result).await.unwrap(), 2);
        assert_eq!(result, vec![0x00, 0x00]);
    }

    #[async_std::test]
    async fn decode_empty() {
        let mut test_stream = AsyncCursor::new([0x00, 0x00]);
        assert_eq!(
            read_utf8_string(&mut test_stream).await.unwrap(),
            String::default()
        );
    }

    #[async_std::test]
    async fn decode() {
        let mut test_stream = AsyncCursor::new([0x00, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94]);
        assert_eq!(
            read_utf8_string(&mut test_stream).await.unwrap(),
            String::from("A𪛔")
        );
    }

    #[async_std::test]
    async fn decode_eof() {
        let mut test_stream = AsyncCursor::new([0x00, 0x05, 0x41]);
        assert_matches!(
            read_utf8_string(&mut test_stream).await,
            Err(Error::MalformedPacket)
        );
    }
}
