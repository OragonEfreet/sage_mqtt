use crate::{codec, Error as SageError, QoS, Result as SageResult};
use futures::io::{AsyncRead, AsyncWrite};
use std::marker::Unpin;

/// Write the given `QoS` instance in one byte.
/// In case of success, returns `1`.
pub async fn write_qos<W: AsyncWrite + Unpin>(qos: QoS, writer: &mut W) -> SageResult<usize> {
    codec::write_byte(qos as u8, writer).await
}

///Read the given reader for a `QoS`, returning it in case of success.
pub async fn read_qos<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<QoS> {
    match codec::read_byte(reader).await? {
        0x00 => Ok(QoS::AtMostOnce),
        0x01 => Ok(QoS::AtLeastOnce),
        0x02 => Ok(QoS::ExactlyOnce),
        _ => Err(SageError::ProtocolError),
    }
}

#[cfg(test)]
mod unit {

    use async_std::io::Cursor;

    use super::*;

    #[async_std::test]
    async fn encode() {
        for (qos, byte) in &[
            (QoS::AtMostOnce, 0x00u8),
            (QoS::AtLeastOnce, 0x01u8),
            (QoS::ExactlyOnce, 0x02u8),
        ] {
            let mut result = Vec::new();
            assert_eq!(write_qos(*qos, &mut result).await.unwrap(), 1);
            assert_eq!(result[0], *byte);
        }
    }

    #[async_std::test]
    async fn decode() {
        for (qos, byte) in &[
            (QoS::AtMostOnce, 0x00u8),
            (QoS::AtLeastOnce, 0x01u8),
            (QoS::ExactlyOnce, 0x02u8),
        ] {
            let mut test_stream = Cursor::new([*byte]);
            let result = read_qos(&mut test_stream).await.unwrap();
            assert_eq!(result, *qos);
        }
    }
}
