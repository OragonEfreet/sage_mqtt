use crate::{
    codec,
    defaults::{
        DEFAULT_MAXIMUM_QOS, DEFAULT_RECEIVE_MAXIMUM, DEFAULT_RETAIN_AVAILABLE,
        DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE, DEFAULT_SUBSCRIPTION_IDENTIFIER_AVAILABLE,
        DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
    },
    Authentication, ClientID, Connect, PropertiesDecoder, Property, QoS,
    ReasonCode::{self, ProtocolError},
    Result as SageResult,
};
use futures::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use std::{
    convert::{From, TryInto},
    marker::Unpin,
};

/// The `Connack` message is sent from the server to the client to acknowledge
/// the connection request. This can be the direct response to a `Connect`
/// message or the closing exchange of `Connack` packets.
#[derive(PartialEq, Debug, Clone)]
pub struct ConnAck {
    /// If the `session_present` is true, the connection is accepted using a
    /// previously and unexpired session.
    pub session_present: bool,

    /// The reason code for the connect acknowledgement.
    /// - `Success`
    /// - `UnspecifiedError`
    /// - `MalformedPacket`
    /// - `ProtocolError`
    /// - `ImplementationSpecificError`
    /// - `UnsupportedProtocolVersion`
    /// - `ClientIdentifierNotValid`
    /// - `BadUserNameOrPassword`
    /// - `NotAuthorized`
    /// - `ServerUnavailable`
    /// - `ServerBusy`
    pub reason_code: ReasonCode,

    /// The session expiry interval the server will use. If absent the server
    /// simply accepted the value sent by the server in the `Connect` packet.
    pub session_expiry_interval: Option<u32>,

    /// The maximum number of `AtLeastOnce` and `ExactlyOnce` qualities of
    /// service the server will concurrently treat for the client.
    pub receive_maximum: u16,

    /// The maximum quality of service the server is willing to accept.
    /// This value cannot be `ExactlyOnce`. Any server receiving a message
    /// with QoS higher than it's maximum is expected to close the connection.
    pub maximum_qos: QoS,

    /// `true` if the server supports retaining messages. `false` otherwise.
    /// Sending retain messages (including Last Will) to a server which does not
    /// support this feature will resulting in a disconnection.
    pub retain_available: bool,

    /// The maximum size in bytes the server is willing to accept. This value
    /// cannot be `0`. If absent there is not maximum packet size.
    pub maximum_packet_size: Option<u32>,

    /// If the `Connect` packet did not have any client id, the server will
    /// send one using `assigned_client_id`.
    pub assigned_client_id: Option<ClientID>,

    /// The maximum value the server will accept as topic alias. If `0` the
    /// server does not accept topic aliases.
    pub topic_alias_maximum: u16,

    /// A human readable reason string used to describe the connack. This field
    /// is optional.
    pub reason_string: String,

    /// General purpose user properties.
    pub user_properties: Vec<(String, String)>,

    /// If `true`, the server accepts subscribing to topics using wildcards.
    pub wildcard_subscription_available: bool,

    /// If `true`, the server accepts subscription identifiers.
    pub subscription_identifiers_available: bool,

    /// If `true`, the server accepts shared subscriptions.
    pub shared_subscription_available: bool,

    /// The server can override the keep alive value requested by the client
    /// upon `Connect`.
    pub keep_alive: Option<u16>,

    /// If the client asked for response information, the server may send it
    /// in `response_information`.
    /// The response information can be used by the client as an hint to
    /// generate reponse topic when making Request/Reponse messages.
    pub response_information: Option<String>,

    /// If the reason code is `ServerMoved` or `UserAnotherServer`, the
    /// `reference` field is used to inform the client about why new server to
    /// connect to instead.
    pub reference: Option<String>,

    /// Upon using enhanced connexion, ending the `Connack` exchange will result in
    /// a `ConnAck` packet which may contain `Authentication` data.
    pub authentication: Option<Authentication>,
}

impl Default for ConnAck {
    fn default() -> Self {
        ConnAck {
            session_present: false,
            reason_code: ReasonCode::Success,
            session_expiry_interval: None,
            receive_maximum: DEFAULT_RECEIVE_MAXIMUM,
            maximum_qos: DEFAULT_MAXIMUM_QOS,
            retain_available: DEFAULT_RETAIN_AVAILABLE,
            maximum_packet_size: None,
            assigned_client_id: None,
            topic_alias_maximum: DEFAULT_TOPIC_ALIAS_MAXIMUM,
            reason_string: Default::default(),
            user_properties: Default::default(),
            wildcard_subscription_available: DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
            subscription_identifiers_available: DEFAULT_SUBSCRIPTION_IDENTIFIER_AVAILABLE,
            shared_subscription_available: DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
            keep_alive: None,
            response_information: Default::default(),
            reference: None,
            authentication: None,
        }
    }
}

impl ConnAck {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_bool(self.session_present, writer).await?;
        n_bytes += codec::write_reason_code(self.reason_code, writer).await?;

        let mut properties = Vec::new();

        if let Some(v) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(v)
                .encode(&mut properties)
                .await?;
        }
        n_bytes += Property::ReceiveMaximum(self.receive_maximum)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::MaximumQoS(self.maximum_qos)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::RetainAvailable(self.retain_available)
            .encode(&mut properties)
            .await?;
        if let Some(v) = self.maximum_packet_size {
            n_bytes += Property::MaximumPacketSize(v)
                .encode(&mut properties)
                .await?;
        }
        if let Some(v) = self.assigned_client_id {
            n_bytes += Property::AssignedClientIdentifier(v)
                .encode(&mut properties)
                .await?;
        }
        n_bytes += Property::TopicAliasMaximum(self.topic_alias_maximum)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::ReasonString(self.reason_string)
            .encode(&mut properties)
            .await?;
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }
        n_bytes += Property::WildcardSubscriptionAvailable(self.wildcard_subscription_available)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::SharedSubscriptionAvailable(self.shared_subscription_available)
            .encode(&mut properties)
            .await?;
        if let Some(v) = self.keep_alive {
            n_bytes += Property::ServerKeepAlive(v).encode(&mut properties).await?;
        }

        if let Some(v) = self.response_information {
            n_bytes += Property::ResponseInformation(v)
                .encode(&mut properties)
                .await?;
        }

        if let Some(v) = self.reference {
            n_bytes += Property::ServerReference(v).encode(&mut properties).await?;
        }
        if let Some(authentication) = self.authentication {
            n_bytes += Property::AuthenticationMethod(authentication.method)
                .encode(&mut properties)
                .await?;
            if !authentication.data.is_empty() {
                n_bytes += Property::AuthenticationData(authentication.data)
                    .encode(&mut properties)
                    .await?;
            }
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
        writer.write_all(&properties).await?;

        Ok(n_bytes)
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let session_present = codec::read_bool(reader).await?;

        let reason_code = codec::read_byte(reader).await?.try_into()?;

        let mut session_expiry_interval = None;
        let mut receive_maximum = DEFAULT_RECEIVE_MAXIMUM;
        let mut maximum_qos = DEFAULT_MAXIMUM_QOS;
        let mut retain_available = DEFAULT_RETAIN_AVAILABLE;
        let mut maximum_packet_size = None;
        let mut assigned_client_id = None;
        let mut topic_alias_maximum = DEFAULT_TOPIC_ALIAS_MAXIMUM;
        let mut reason_string = Default::default();
        let mut user_properties = Vec::new();
        let mut wildcard_subscription_available = DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE;
        let mut subscription_identifiers_available = DEFAULT_SUBSCRIPTION_IDENTIFIER_AVAILABLE;
        let mut shared_subscription_available = DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE;
        let mut keep_alive = None;
        let mut response_information = None;
        let mut reference = None;
        let mut authentication_method = None;
        let mut authentication_data = Default::default();

        let mut decoder = PropertiesDecoder::take(reader).await?;
        while decoder.has_properties() {
            match decoder.read().await? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = Some(v),
                Property::ReceiveMaximum(v) => receive_maximum = v,
                Property::MaximumQoS(v) => maximum_qos = v,
                Property::RetainAvailable(v) => retain_available = v,
                Property::MaximumPacketSize(v) => maximum_packet_size = Some(v),
                Property::AssignedClientIdentifier(v) => assigned_client_id = Some(v),
                Property::TopicAliasMaximum(v) => topic_alias_maximum = v,
                Property::ReasonString(v) => reason_string = v,
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                Property::WildcardSubscriptionAvailable(v) => wildcard_subscription_available = v,
                Property::SubscriptionIdentifiersAvailable(v) => {
                    subscription_identifiers_available = v
                }
                Property::SharedSubscriptionAvailable(v) => shared_subscription_available = v,
                Property::ServerKeepAlive(v) => keep_alive = Some(v),
                Property::ResponseInformation(v) => response_information = Some(v),
                Property::ServerReference(v) => reference = Some(v),
                Property::AuthenticationMethod(v) => authentication_method = Some(v),
                Property::AuthenticationData(v) => authentication_data = v,
                _ => return Err(ProtocolError.into()),
            }
        }

        let authentication = if let Some(method) = authentication_method {
            Some(Authentication {
                method,
                data: authentication_data,
            })
        } else {
            if !authentication_data.is_empty() {
                return Err(ProtocolError.into());
            }
            None
        };

        Ok(ConnAck {
            session_present,
            reason_code,
            session_expiry_interval,
            receive_maximum,
            maximum_qos,
            retain_available,
            maximum_packet_size,
            assigned_client_id,
            topic_alias_maximum,
            reason_string,
            user_properties,
            wildcard_subscription_available,
            subscription_identifiers_available,
            shared_subscription_available,
            keep_alive,
            response_information,
            reference,
            authentication,
        })
    }
}

impl From<Connect> for ConnAck {
    fn from(connect: Connect) -> Self {
        ConnAck {
            reason_code: ReasonCode::Success,
            session_expiry_interval: Some(connect.session_expiry_interval),
            maximum_packet_size: connect.maximum_packet_size,
            assigned_client_id: connect.client_id,
            topic_alias_maximum: connect.topic_alias_maximum,
            keep_alive: Some(connect.keep_alive),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use async_std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            1, 138, 111, 17, 0, 0, 5, 57, 33, 0, 30, 36, 1, 37, 0, 39, 0, 0, 1, 0, 18, 0, 11, 87,
            97, 108, 107, 84, 104, 105, 115, 87, 97, 121, 34, 0, 10, 31, 0, 7, 82, 85, 78, 45, 68,
            77, 67, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 40, 0, 42, 0, 19,
            0, 17, 26, 0, 9, 65, 101, 114, 111, 115, 109, 105, 116, 104, 28, 0, 14, 80, 97, 105,
            110, 116, 32, 73, 116, 32, 66, 108, 97, 99, 107, 21, 0, 6, 87, 105, 108, 108, 111, 119,
            22, 0, 4, 13, 21, 234, 94,
        ]
    }

    fn decoded() -> ConnAck {
        ConnAck {
            session_present: true,
            reason_code: ReasonCode::Banned,
            session_expiry_interval: Some(1337),
            receive_maximum: 30,
            maximum_qos: QoS::AtLeastOnce,
            retain_available: false,
            maximum_packet_size: Some(256),
            assigned_client_id: Some("WalkThisWay".into()),
            topic_alias_maximum: 10,
            reason_string: "RUN-DMC".into(),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            wildcard_subscription_available: false,
            subscription_identifiers_available: true,
            shared_subscription_available: false,
            keep_alive: Some(17),
            response_information: Some("Aerosmith".into()),
            reference: Some("Paint It Black".into()),
            authentication: Some(Authentication {
                method: "Willow".into(),
                data: vec![0x0D, 0x15, 0xEA, 0x5E],
            }),
        }
    }

    #[async_std::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 114);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = ConnAck::read(&mut test_data).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
