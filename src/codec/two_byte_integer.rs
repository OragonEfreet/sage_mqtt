use crate::{Error, Result as SageResult};
use async_std::io::{prelude::*, Read, Write};
use std::marker::Unpin;

/// Writes the given `u16` according to MQTT5 Two Byte Integer specifications.
/// In case of success, returns `2`.
pub async fn write_two_byte_integer<W: Write + Unpin>(
    data: u16,
    writer: &mut W,
) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes()).await?)
}

/// Reads the given `reader` for an `u16`, returning it in case of success.
pub async fn read_two_byte_integer<R: Read + Unpin>(reader: &mut R) -> SageResult<u16> {
    let mut buf = [0_u8; 2];
    if reader.read_exact(&mut buf).await.is_ok() {
        Ok(((buf[0] as u16) << 8) | buf[1] as u16)
    } else {
        Err(Error::MalformedPacket)
    }
}

#[cfg(test)]
mod unit {

    use async_std::io::Cursor;

    use super::*;

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
        assert_matches!(
            read_two_byte_integer(&mut test_stream).await,
            Err(Error::MalformedPacket)
        );
    }
}
