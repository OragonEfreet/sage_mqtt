use crate::{
    codec, PropertiesDecoder, Property,
    ReasonCode::{self, ProtocolError},
    Result as SageResult,
};
use std::{convert::TryInto, marker::Unpin};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// The `SubAck` packet is sent by a server to confirm a `Subscribe` has been
/// received and processed.
#[derive(Debug, PartialEq, Clone)]
pub struct SubAck {
    /// The packet identifier is used to identify the message throughout the
    /// communication.
    pub packet_identifier: u16,

    /// User defined properties
    pub user_properties: Vec<(String, String)>,

    /// The reason codes. The array contains one `ReasonCode` per subscription.
    /// The indices in this array match the incides in the `Subscribe`'s
    /// subscriptions array.
    pub reason_codes: Vec<ReasonCode>,
}

impl Default for SubAck {
    fn default() -> Self {
        SubAck {
            packet_identifier: 0,
            user_properties: Default::default(),
            reason_codes: Default::default(),
        }
    }
}

impl SubAck {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, mut writer: W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, &mut writer).await?;

        let mut properties = Vec::new();

        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, &mut writer).await?;
        writer.write_all(&properties).await?;

        for reason_code in self.reason_codes {
            n_bytes += codec::write_reason_code(reason_code, &mut writer).await?;
        }

        Ok(n_bytes)
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(
        reader: R,
        remaining_size: usize,
    ) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);

        let packet_identifier = codec::read_two_byte_integer(&mut reader).await?;
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(&mut reader).await?;
        while properties.has_properties() {
            match properties.read().await? {
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(ProtocolError.into()),
            }
        }

        let mut reason_codes = Vec::new();

        while reader.limit() > 0 {
            reason_codes.push(codec::read_byte(&mut reader).await?.try_into()?);
        }

        Ok(SubAck {
            packet_identifier,
            user_properties,
            reason_codes,
        })
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 15, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 145, 143,
        ]
    }

    fn decoded() -> SubAck {
        SubAck {
            packet_identifier: 1337,
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            reason_codes: vec![
                ReasonCode::PacketIdentifierInUse,
                ReasonCode::TopicFilterInvalid,
            ],
        }
    }

    #[tokio::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 20);
    }

    #[tokio::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = SubAck::read(&mut test_data, 20).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
