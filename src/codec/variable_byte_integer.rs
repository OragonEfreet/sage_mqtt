use crate::{ReasonCode::MalformedPacket, Result as SageResult};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

///Write the given `u32` into `writer` according to MQTT5 Variable Byte Integer
/// specifications, returning the number of bytes written (`1`, `2`, `3` or `4`)
/// in case of success.
pub async fn write_variable_byte_integer<W: AsyncWrite + Unpin>(
    data: u32,
    writer: &mut W,
) -> SageResult<usize> {
    let mut n_encoded_bytes = 0;
    let mut x = data;
    loop {
        let mut encoded_byte = (x % 128) as u8;
        x /= 128;
        if x > 0 {
            encoded_byte |= 128u8;
        }
        n_encoded_bytes += writer.write(&[encoded_byte]).await?;
        if x == 0 {
            break;
        }
    }
    Ok(n_encoded_bytes)
}

///Read the given stream for a `u32` encoded as Variable Byte Integer.
/// Returns the read value in case of success.
pub async fn read_variable_byte_integer<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<u32> {
    let mut multiplier = 1_u32;
    let mut value = 0_u32;

    loop {
        let mut buffer = vec![0u8; 1];
        reader.read_exact(&mut buffer).await?;
        let encoded_byte = buffer[0];
        value += ((encoded_byte & 127u8) as u32) * multiplier;
        if multiplier > 2_097_152 {
            return Err(MalformedPacket.into());
        }
        multiplier *= 128;
        if encoded_byte & 128u8 == 0 {
            break;
        }
    }

    Ok(value)
}

#[cfg(test)]
mod unit {

    use super::*;
    use crate::Error;
    use std::io::{Cursor, ErrorKind};

    // The encoded value MUST use the minimum number of bytes necessary to
    // represent the value
    // Note: This test considers the fact that if VALUE_L and VALUE_R are
    // both encoded into N bytes, then all values between VALUE_L and VALUE_R
    // are encoded into N bytes as well. Meaning: we only check bounds.
    #[tokio::test]
    async fn mqtt_1_5_5_1() {
        let bounds = [
            [0u32, 12],
            [128, 16_383],
            [16_384, 2_097_151],
            [2_097_152, 268_435_455],
        ];

        let mut result = Vec::new();

        let mut expected_buffer_size = 1;

        for bound in &bounds {
            for i in bound {
                let n_bytes = write_variable_byte_integer(*i, &mut result).await.unwrap();
                assert_eq!(
                    n_bytes, expected_buffer_size,
                    "Variable Byte Integer '{}' should be encoded to '{}' bytes. Used '{}' instead",
                    i, expected_buffer_size, n_bytes
                );
                result.clear();
            }

            expected_buffer_size += 1;
        }
    }

    #[tokio::test]
    async fn encode_one_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(0u32, &mut result)
                .await
                .unwrap(),
            1
        );
        assert_eq!(result, vec![0x00]);
    }

    #[tokio::test]
    async fn encode_one_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(127u32, &mut result)
                .await
                .unwrap(),
            1
        );
        assert_eq!(result, vec![0x7F]);
    }

    #[tokio::test]
    async fn encode_two_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(128u32, &mut result)
                .await
                .unwrap(),
            2
        );
        assert_eq!(result, vec![0x80, 0x01]);
    }

    #[tokio::test]
    async fn encode_two_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(16_383u32, &mut result)
                .await
                .unwrap(),
            2
        );
        assert_eq!(result, vec![0xFF, 0x7F]);
    }

    #[tokio::test]
    async fn encode_three_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(16_384u32, &mut result)
                .await
                .unwrap(),
            3
        );
        assert_eq!(result, vec![0x80, 0x80, 0x01]);
    }

    #[tokio::test]
    async fn encode_three_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(2_097_151u32, &mut result)
                .await
                .unwrap(),
            3
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0x7F]);
    }

    #[tokio::test]
    async fn encode_four_lower_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(2_097_152u32, &mut result)
                .await
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0x80, 0x80, 0x80, 0x01]);
    }

    #[tokio::test]
    async fn encode_four_upper_bound() {
        let mut result = Vec::new();
        assert_eq!(
            write_variable_byte_integer(268_435_455u32, &mut result)
                .await
                .unwrap(),
            4
        );
        assert_eq!(result, vec![0xFF, 0xFF, 0xFF, 0x7F]);
    }

    #[tokio::test]
    async fn decode_one_lower_bound() {
        let mut test_stream = Cursor::new([0x00]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            0u32
        );
    }

    #[tokio::test]
    async fn decode_one_upper_bound() {
        let mut test_stream = Cursor::new([0x7F]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            127u32
        );
    }

    #[tokio::test]
    async fn decode_two_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x01]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            128u32
        );
    }

    #[tokio::test]
    async fn decode_two_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0x7F]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            16_383u32
        );
    }

    #[tokio::test]
    async fn decode_three_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x01]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            16_384u32
        );
    }

    #[tokio::test]
    async fn decode_three_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0x7F]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            2_097_151u32
        );
    }

    #[tokio::test]
    async fn decode_four_lower_bound() {
        let mut test_stream = Cursor::new([0x80, 0x80, 0x80, 0x01]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            2_097_152u32
        );
    }

    #[tokio::test]
    async fn decode_four_upper_bound() {
        let mut test_stream = Cursor::new([0xFF, 0xFF, 0xFF, 0x7F]);
        assert_eq!(
            read_variable_byte_integer(&mut test_stream).await.unwrap(),
            268_435_455u32
        );
    }

    #[tokio::test]
    async fn decode_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        let result = read_variable_byte_integer(&mut test_stream).await;
        if let Some(Error::Io(err)) = result.err() {
            assert!(matches!(err.kind(), ErrorKind::UnexpectedEof));
        } else {
            panic!("Should be IO Error");
        }
    }
}
