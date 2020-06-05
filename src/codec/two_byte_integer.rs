use crate::Result as SageResult;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// Write the given `u16` according to MQTT5 Two Byte Integer specifications.
/// In case of success, returns `2`.
pub async fn write_two_byte_integer<W: AsyncWrite + Unpin>(
    data: u16,
    writer: &mut W,
) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes()).await?)
}

/// Read the given `reader` for an `u16`, returning it in case of success.
pub async fn read_two_byte_integer<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<u16> {
    let mut buf = [0_u8; 2];

    reader.read_exact(&mut buf).await?;
    Ok(((buf[0] as u16) << 8) | buf[1] as u16)
}

#[cfg(test)]
mod unit {

    use super::*;
    use crate::Error;
    use async_std::io::Cursor;
    use futures::io::ErrorKind;

    #[async_std::test]
    async fn encode() {
        let mut result = Vec::new();
        assert_eq!(
            write_two_byte_integer(1984u16, &mut result).await.unwrap(),
            2
        );
        assert_eq!(result, vec![0x07, 0xC0]);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_stream = Cursor::new([0x07, 0xC0]);
        assert_eq!(
            read_two_byte_integer(&mut test_stream).await.unwrap(),
            1984u16
        );
    }

    #[async_std::test]
    async fn decode_eof() {
        let mut test_stream = Cursor::new([0x07]);
        let result = read_two_byte_integer(&mut test_stream).await;
        if let Some(Error::Io(err)) = result.err() {
            assert_matches!(err.kind(), ErrorKind::UnexpectedEof);
        } else {
            panic!("Should be IO Error");
        }
    }
}
