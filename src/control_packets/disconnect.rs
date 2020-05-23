use crate::{
    codec, ControlPacketType, Error, PropertiesDecoder, Property, ReasonCode, Result as SageResult,
};
use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use std::marker::Unpin;

/// A `Disconnect` packet can be sent by the client or the server to gracefully
/// disconnect.
#[derive(Debug, PartialEq, Clone)]
pub struct Disconnect {
    /// The reason code code the `Disconnect` notice.can be any of:
    /// - Client or Server
    ///   + `AdministrativeAction`
    ///   + `ImplementationSpecificError`
    ///   + `MalformedPacket`
    ///   + `MessageRateTooHigh`
    ///   + `NormalDisconnection`
    ///   + `PacketTooLarge`
    ///   + `PayloadFormatInvalid`
    ///   + `ProtocolError`
    ///   + `QuotaExceeded`
    ///   + `ReceiveMaximumExceeded`
    ///   + `TopicAliasInvalid`
    ///   + `TopicNameInvalid`
    ///   + `UnspecifiedError`
    /// - Server Only
    ///   + `ConnectionRateExceeded`
    ///   + `KeepAliveTimeout`
    ///   + `MaximumConnectTime`
    ///   + `NotAuthorized`
    ///   + `QoSNotSupported`
    ///   + `RetainNotSupported`
    ///   + `ServerBusy`
    ///   + `ServerMoved`
    ///   + `ServerShuttingDown`
    ///   + `SessionTakenOver`
    ///   + `SharedSubscriptionsNotSupported`
    ///   + `SubscriptionIdentifiersNotSupported`
    ///   + `TopicFilterInvalid`
    ///   + `UseAnotherServer`
    ///   + `WildcardSubscriptionsNotSupported`
    /// - Client Only
    ///   + `DisconnectWithWillMessage`
    pub reason_code: ReasonCode,

    /// `session_expiry_interval` can be used to override the session expiry
    /// period formerly set upon connection. If not present, the session expiry
    /// interval value set using `Connect` or `Connack` is still in use.
    pub session_expiry_interval: Option<u32>,

    /// An optional descriptin of the reason for deconnecting.
    pub reason_string: Option<String>,

    /// General purpose user properties.
    pub user_properties: Vec<(String, String)>,

    /// If the reason code is `ServerMoved` or `UserAnotherServer`, the
    /// `reference` field is used to inform the client about why new server to
    /// connect to instead.
    pub reference: Option<String>,
}

impl Default for Disconnect {
    fn default() -> Self {
        Disconnect {
            reason_code: ReasonCode::Success,
            reason_string: None,
            session_expiry_interval: None,
            user_properties: Default::default(),
            reference: None,
        }
    }
}

impl Disconnect {
    ///Write the `Disconnect` body of a packet, returning the written size in bytes
    /// in case of success.
    pub async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_reason_code(self.reason_code, writer).await?;

        let mut properties = Vec::new();

        if let Some(v) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(v)
                .encode(&mut properties)
                .await?;
        }
        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties).await?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }
        if let Some(v) = self.reference {
            n_bytes += Property::ServerReference(v).encode(&mut properties).await?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
        writer.write_all(&properties).await?;

        Ok(n_bytes)
    }

    ///Read the `Disconnect` body from `reader`, retuning it in case of success.
    pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let reason_code = ReasonCode::try_parse(
            codec::read_byte(reader).await?,
            ControlPacketType::DISCONNECT,
        )?;
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(reader).await?;
        let mut session_expiry_interval = None;
        let mut reason_string = None;
        let mut reference = None;

        while properties.has_properties() {
            match properties.read().await? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = Some(v),
                Property::ReasonString(v) => reason_string = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                Property::ServerReference(v) => reference = Some(v),
                _ => return Err(Error::ProtocolError),
            }
        }

        Ok(Disconnect {
            reason_code,
            session_expiry_interval,
            reason_string,
            user_properties,
            reference,
        })
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use async_std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            150, 74, 17, 0, 0, 5, 57, 31, 0, 22, 76, 111, 115, 101, 32, 89, 111, 117, 114, 115,
            101, 108, 102, 32, 116, 111, 32, 68, 97, 110, 99, 101, 38, 0, 4, 68, 97, 102, 116, 0,
            4, 80, 117, 110, 107, 38, 0, 8, 80, 104, 97, 114, 114, 101, 108, 108, 0, 8, 87, 105,
            108, 108, 105, 97, 109, 115, 28, 0, 7, 67, 111, 109, 101, 32, 111, 110,
        ]
    }

    fn decoded() -> Disconnect {
        Disconnect {
            reason_code: ReasonCode::MessageRateTooHigh,
            session_expiry_interval: Some(1337),
            reason_string: Some("Lose Yourself to Dance".into()),
            user_properties: vec![
                ("Daft".into(), "Punk".into()),
                ("Pharrell".into(), "Williams".into()),
            ],
            reference: Some("Come on".into()),
        }
    }

    #[async_std::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 76);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Disconnect::read(&mut test_data).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
