use crate::{Error, Result as SageResult};
use async_std::io::{prelude::*, Read, Write};
use std::marker::Unpin;

/// Writes the given `u32` according to MQTT5 Four Byte Integer specifications.
/// In case of success, returns `4`.
pub async fn write_four_byte_integer<W: Write + Unpin>(
    data: u32,
    writer: &mut W,
) -> SageResult<usize> {
    Ok(writer.write(&data.to_be_bytes()).await?)
}

/// Reads the given `reader` for an `u32`, returning it in case of success.
pub async fn read_four_byte_integer<R: Read + Unpin>(reader: &mut R) -> SageResult<u32> {
    let mut buf = [0_u8; 4];
    if reader.read_exact(&mut buf).await.is_ok() {
        Ok(((buf[0] as u32) << 24)
            | ((buf[1] as u32) << 16)
            | ((buf[2] as u32) << 8)
            | (buf[3] as u32))
    } else {
        Err(Error::MalformedPacket)
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use async_std::io::Cursor;

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
        assert_matches!(
            read_four_byte_integer(&mut test_stream).await,
            Err(Error::MalformedPacket)
        );
    }
}
