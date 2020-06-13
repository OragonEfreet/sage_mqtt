use crate::{codec, Error, PropertiesDecoder, Property, ReasonCode, Result as SageResult};
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// An `Unsubscribe` packet is sent from the client to unsubsribe to a topic.
#[derive(Debug, PartialEq, Clone)]
pub struct UnSubscribe {
    /// The packet identifier is used to identify the message throughout the
    /// communication.
    pub packet_identifier: u16,

    /// General purpose user-properties
    pub user_properties: Vec<(String, String)>,

    /// The list of topics to unsubsribe to. They can contains wildcards.
    pub subscriptions: Vec<String>,
}

impl Default for UnSubscribe {
    fn default() -> Self {
        UnSubscribe {
            packet_identifier: 0,
            user_properties: Default::default(),
            subscriptions: Default::default(),
        }
    }
}

impl UnSubscribe {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, writer).await?;

        let mut properties = Vec::new();
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }
        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
        writer.write_all(&properties).await?;

        for option in self.subscriptions {
            n_bytes += codec::write_utf8_string(&option, writer).await?;
        }

        Ok(n_bytes)
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(
        reader: &mut R,
        remaining_size: usize,
    ) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);

        let packet_identifier = codec::read_two_byte_integer(&mut reader).await?;

        let mut user_properties = Vec::new();

        let mut properties = PropertiesDecoder::take(&mut reader).await?;
        while properties.has_properties() {
            match properties.read().await? {
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::Reason(ReasonCode::ProtocolError)),
            }
        }

        let mut subscriptions = Vec::new();

        while reader.limit() > 0 {
            subscriptions.push(codec::read_utf8_string(&mut reader).await?);
        }

        if subscriptions.is_empty() {
            Err(Error::Reason(ReasonCode::ProtocolError))
        } else {
            Ok(UnSubscribe {
                packet_identifier,
                user_properties,
                subscriptions,
            })
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use async_std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 15, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 0, 6, 104, 97,
            114, 100, 101, 114, 0, 6, 98, 101, 116, 116, 101, 114, 0, 6, 102, 97, 115, 116, 101,
            114, 0, 8, 115, 116, 114, 111, 110, 103, 101, 114,
        ]
    }

    fn decoded() -> UnSubscribe {
        UnSubscribe {
            packet_identifier: 1337,
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            subscriptions: vec![
                "harder".into(),
                "better".into(),
                "faster".into(),
                "stronger".into(),
            ],
        }
    }

    #[async_std::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 52);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = UnSubscribe::read(&mut test_data, 52).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
