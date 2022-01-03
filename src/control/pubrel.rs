use crate::{
    codec, PropertiesDecoder, Property,
    ReasonCode::{self, ProtocolError},
    Result as SageResult,
};
use std::{convert::TryInto, marker::Unpin};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

/// The `PubRel` packet is sent during an `ExactlyOnce` quality of service
/// publish.
#[derive(Debug, PartialEq, Clone)]
pub struct PubRel {
    /// The packet identifier is used to identify the message throughout the
    /// communication.
    pub packet_identifier: u16,

    /// The reason code for the acknowledgement. Can be any of:
    /// - `Success`
    /// - `NoMatchingSubscribers`
    /// - `UnspecifiedError`
    /// - `ImplementationSpecificError`
    /// - `NotAuthorized`
    /// - `TopicNameInvalid`
    /// - `PacketIdentifierInUse`
    /// - `QuotaExceeded`
    /// - `PayloadFormatInvalid`
    pub reason_code: ReasonCode,

    /// If available, the reason string describing the acknowledgement.
    pub reason_string: Option<String>,

    /// General purpose user properties
    pub user_properties: Vec<(String, String)>,
}

impl Default for PubRel {
    fn default() -> Self {
        PubRel {
            packet_identifier: 0,
            reason_code: ReasonCode::Success,
            reason_string: None,
            user_properties: Default::default(),
        }
    }
}

impl PubRel {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, writer).await?;

        let mut properties = Vec::new();

        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties).await?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }

        if n_bytes == 2 && self.reason_code != ReasonCode::Success {
            Ok(2)
        } else {
            n_bytes += codec::write_reason_code(self.reason_code, writer).await?;
            n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
            writer.write_all(&properties).await?;
            Ok(n_bytes)
        }
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(
        reader: &mut R,
        shortened: bool,
    ) -> SageResult<Self> {
        let packet_identifier = codec::read_two_byte_integer(reader).await?;

        let mut pubrel = PubRel {
            packet_identifier,
            ..Default::default()
        };

        if shortened {
            pubrel.reason_code = ReasonCode::Success;
        } else {
            pubrel.reason_code = codec::read_byte(reader).await?.try_into()?;

            let mut properties = PropertiesDecoder::take(reader).await?;
            while properties.has_properties() {
                match properties.read().await? {
                    Property::ReasonString(v) => pubrel.reason_string = Some(v),
                    Property::UserProperty(k, v) => pubrel.user_properties.push((k, v)),
                    _ => return Err(ProtocolError.into()),
                }
            }
        }

        Ok(pubrel)
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 146, 29, 31, 0, 11, 66, 108, 97, 99, 107, 32, 66, 101, 116, 116, 121, 38, 0, 7,
            77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116,
        ]
    }

    fn decoded() -> PubRel {
        PubRel {
            packet_identifier: 1337,
            reason_code: ReasonCode::PacketIdentifierNotFound,
            reason_string: Some("Black Betty".into()),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
        }
    }

    #[tokio::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 33);
    }

    #[tokio::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = PubRel::read(&mut test_data, false).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
