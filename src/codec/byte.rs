use crate::{Error, Result as SageResult};
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// Writes the given byte into `writer`.
/// In case of success, returns `1`
pub async fn write_byte<W: AsyncWrite + Unpin>(byte: u8, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[byte]).await?)
}

/// Writes the given bool into `writer` in a single byte value.
/// MQTT5 specifications do not define an actual boolean type but expresses it
/// with a byte being `0x00` for `false` or `0x01` for `false`. Other values are
/// considered incorrect.
/// In case of success, returns `1`
pub async fn write_bool<W: AsyncWrite + Unpin>(data: bool, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[data as u8]).await?)
}

/// Reads the given `reader` for a byte value.
/// In case of success, returns an `u8`
pub async fn read_byte<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<u8> {
    let mut buf = [0u8; 1];
    if reader.read_exact(&mut buf).await.is_ok() {
        Ok(buf[0])
    } else {
        Err(Error::MalformedPacket)
    }
}

/// Reads the given `reader` for a boolean value.
/// MQTT5 specifications do not define an actual boolean type but expresses it
/// with a byte being `0x00` for `false` or `0x01` for `false`. Other values are
/// considered incorrect.
/// In case of success, returns an `bool`
pub async fn read_bool<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<bool> {
    let byte = read_byte(reader).await?;
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::ProtocolError),
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use async_std::io::Cursor;

    #[async_std::test]
    async fn encode() {
        let mut buffer = Vec::new();
        let result = write_byte(0b00101010, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x2A]);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        let result = read_byte(&mut test_stream).await.unwrap();
        assert_eq!(result, 0xAF);
    }

    #[async_std::test]
    async fn decode_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        let result = read_byte(&mut test_stream).await;
        assert_matches!(result, Err(Error::MalformedPacket));
    }

    #[async_std::test]
    async fn encode_true() {
        let mut buffer = Vec::new();
        let result = write_bool(true, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x01]);
    }

    #[async_std::test]
    async fn encode_false() {
        let mut buffer = Vec::new();
        let result = write_bool(false, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x00]);
    }

    #[async_std::test]
    async fn decode_true() {
        let mut test_stream = Cursor::new([0x01_u8]);
        let result = read_bool(&mut test_stream).await.unwrap();
        assert_eq!(result, true);
    }

    #[async_std::test]
    async fn decode_false() {
        let mut test_stream = Cursor::new([0x00_u8]);
        let result = read_bool(&mut test_stream).await.unwrap();
        assert_eq!(result, false);
    }
}
