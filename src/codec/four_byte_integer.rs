use crate::Result as SageResult;
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// Write the given `u32` according to MQTT5 Four Byte Integer specifications.
/// In case of success, returns `4`.
pub async fn write_four_byte_integer<W: AsyncWrite + Unpin>(
    data: u32,
    writer: &mut W,
) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes()).await?)
}

/// Read the given `reader` for an `u32`, returning it in case of success.
pub async fn read_four_byte_integer<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<u32> {
    let mut buf = [0_u8; 4];
    reader.read_exact(&mut buf).await?;
    Ok(
        ((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32),
    )
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
            write_four_byte_integer(220_000_u32, &mut result)
                .await
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0x00, 0x03, 0x5B, 0x60]);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_stream = Cursor::new([0x00, 0x03, 0x5B, 0x60]);
        assert_eq!(
            read_four_byte_integer(&mut test_stream).await.unwrap(),
            220_000_u32
        );
    }

    #[async_std::test]
    async fn decode_eof() {
        let mut test_stream = Cursor::new([0x07]);
        let result = read_four_byte_integer(&mut test_stream).await;
        if let Some(Error::Io(err)) = result.err() {
            assert!(matches!(err.kind(), ErrorKind::UnexpectedEof));
        } else {
            panic!("Should be IO Error");
        }
    }
}
