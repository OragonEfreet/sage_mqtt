use crate::{
    codec,
    defaults::{
        DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM,
        DEFAULT_REQUEST_PROBLEM_INFORMATION, DEFAULT_REQUEST_RESPONSE_INFORMATION,
        DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILL_DELAY_INTERVAL,
    },
    Authentication, ClientID, PropertiesDecoder, Property, QoS,
    ReasonCode::{ClientIdentifierNotValid, MalformedPacket, ProtocolError},
    Result as SageResult, Topic, Will,
};
use std::{convert::TryInto, marker::Unpin};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

/// The `Connect` control packet is used to open a session. It is the first
/// Packet a client must send to a server once the connection is established.
/// A _Connect_ packet can only be sent once for each connection.
///
/// # Session and connection
///
/// A unique connection can only send a _Connect_ packet once. If the server
/// received a second _Connect_ packet over a same connection, it is considered
/// as a protocol error.
/// Yet, a same session can continue accross different sequences of connections.
/// In that case, `clean_start` must be set to `false` (default) to continue the
/// session.
///
/// # Client identifier
///
/// The client identifier is a server-unique `String` used to identify the
/// client accross operations. It is possible not to give a client identifier
/// to the server by setting `client_id` to either `None` or an empty string.
/// In that case the server will decide itself for an identifier and return
/// it into the _CONNACK_ packet.
#[derive(PartialEq, Debug, Clone)]
pub struct Connect {
    /// If set, the server will start a new session and drop any existing one
    /// if any.
    pub clean_start: bool,

    /// An optional user name to send to the server.
    pub user_name: Option<String>,

    /// An option password to send to the server.
    pub password: Option<Vec<u8>>,

    /// Specifies the maximum amount of time the client and the server may not
    /// communicate with each other. This value is expressed in seconds.
    /// If the server does not receive any packet from the client in one and
    /// a half times this interval, it will close the connection. Likewise, the
    /// client will close the connection under the same condition. The default
    /// keep alive value is `600` (10mn).
    /// Not that the keep alive mechanism is deactivated if the value is `0`.
    pub keep_alive: u16,

    /// Once the connection is closed, the client and server still keep the
    /// session active during a certain amount of time expressed in seconds.
    /// - If the value is `0` (default) the session ends when the connection is closed.
    /// - If the value is `0xFFFFFFFF` the session never expires.
    /// The client can override the session expiry interval within the
    /// DISCONNECT packet.
    pub session_expiry_interval: Option<u32>,

    /// This value sets the maximum number of _AtLeastOnce_ and _ExactlyOnce_
    /// packets that should be processed concurrently.
    /// There is no such limit for QoS `AtMostOnce` packets.
    /// The default value is `65_535`
    pub receive_maximum: u16,

    /// Defines the maximum size per packet the client is willing to receive
    /// from the server. It is a procotol error to send a packet which size
    /// exceeds this value and the client is expected to disconnect from the
    /// server with a `PacketTooLarge` error.
    /// This value cannot be `0`. Sending or receiving a CONNECT packet with a
    /// `maximum_packet_size` of value `0` is a procotol error.
    /// `maximum_packet_size` is `None` (default), there is no size limit.
    pub maximum_packet_size: Option<u32>,

    /// Topic aliases are a way to reduce the size of packets by substituting
    /// aliases (which are strings) to integer values.
    /// The number of aliases allowed by the client from the server is defined
    /// with the `topic_alias_maximum`. It can be `0`, meaning aliases are
    /// entirely disallowed.
    pub topic_alias_maximum: u16,

    /// This flag can be set to ask the server to send back response information
    /// that can be used as an hint by the client to determine a response topic
    /// used in Request/Response type communication.
    /// This is only an optional hint and the server is allowed not to send any
    /// reponse information even if the value of the field is `true`.
    /// By default, `request_response_information` is `false`.
    pub request_response_information: bool,

    /// In any packet sent by the server that contains a `ReasonCode`, the
    /// latter can be described using a reason string or user properties. These
    /// are called "problem information".
    /// If `request_problem_information` is `true` the server is allowed to
    /// sent problem information in any packet with a `ReasonCode`.
    /// If `false` (default), the server is only allowed to send problem
    /// information on `Publish`, `Connack` and `Disconnect` packets.
    pub request_problem_information: bool,

    /// General purpose properties
    /// By default, a Connect packet has no properties.
    pub user_properties: Vec<(String, String)>,

    /// Define an `Authentication` structure to provide enhanced authentication.
    /// By default, `authentication` is `None`, which means no or basic
    /// authentication using only `user_name` and `password`.
    pub authentication: Option<Authentication>,

    /// The client id is an identifier that uniquely represents the client
    /// from the server point of view. The client id is used to ensure `AtLeastOnce`
    /// and `ExactlyOnce` qualities of service.
    /// A client id is mandatory within a session. Yet, the `Connect` packet
    /// may omit if by setting `client_id` to `None` (default). In that case
    /// the id is created by the server and returns to the client with the
    /// `Connack`  packet.
    pub client_id: Option<ClientID>,

    /// The client's Last Will to send in case of ungraceful disconnection.
    /// This is optional and default is `None`.
    pub will: Option<Will>,
}

impl Default for Connect {
    fn default() -> Self {
        Connect {
            clean_start: false,
            user_name: None,
            password: Default::default(),
            keep_alive: 600,
            session_expiry_interval: None,
            receive_maximum: DEFAULT_RECEIVE_MAXIMUM,
            maximum_packet_size: None,
            topic_alias_maximum: DEFAULT_TOPIC_ALIAS_MAXIMUM,
            request_response_information: DEFAULT_REQUEST_RESPONSE_INFORMATION,
            request_problem_information: DEFAULT_REQUEST_PROBLEM_INFORMATION,
            user_properties: Default::default(),
            authentication: None,
            client_id: None,
            will: None,
        }
    }
}

#[derive(Debug)]
struct ConnectFlags {
    pub clean_start: bool,
    pub will: bool,
    pub will_qos: QoS,
    pub will_retain: bool,
    pub user_name: bool,
    pub password: bool,
}

impl Connect {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, mut writer: W) -> SageResult<usize> {
        // Variable Header (into content)
        let mut n_bytes = codec::write_utf8_string("MQTT", &mut writer).await?;
        n_bytes += codec::write_byte(0x05, &mut writer).await?;

        n_bytes += ConnectFlags {
            clean_start: self.clean_start,
            will: self.will.is_some(),
            will_qos: if let Some(w) = &self.will {
                w.qos
            } else {
                QoS::AtMostOnce
            },
            will_retain: if let Some(w) = &self.will {
                w.retain
            } else {
                false
            },
            user_name: self.user_name.is_some(),
            password: self.password.is_some(),
        }
        .write(&mut writer)
        .await?;

        n_bytes += codec::write_two_byte_integer(self.keep_alive, &mut writer).await?;

        // Properties
        let mut properties = Vec::new();
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(session_expiry_interval)
                .encode(&mut properties)
                .await?;
        }
        n_bytes += Property::ReceiveMaximum(self.receive_maximum)
            .encode(&mut properties)
            .await?;
        if let Some(maximum_packet_size) = self.maximum_packet_size {
            n_bytes += Property::MaximumPacketSize(maximum_packet_size)
                .encode(&mut properties)
                .await?;
        }
        n_bytes += Property::TopicAliasMaximum(self.topic_alias_maximum)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::RequestResponseInformation(self.request_response_information)
            .encode(&mut properties)
            .await?;
        n_bytes += Property::RequestProblemInformation(self.request_problem_information)
            .encode(&mut properties)
            .await?;
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }

        if let Some(authentication) = self.authentication {
            n_bytes += authentication.write(&mut properties).await?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, &mut writer).await?;
        writer.write_all(&properties).await?;

        // Payload
        if let Some(client_id) = self.client_id {
            if client_id.len() > 23 || client_id.chars().any(|c| !('0'..='z').contains(&c)) {
                return Err(MalformedPacket.into());
            }
            n_bytes += codec::write_utf8_string(&client_id, &mut writer).await?;
        } else {
            // Still write empty client id
            n_bytes += codec::write_utf8_string("", &mut writer).await?;
        }

        if let Some(w) = self.will {
            let mut properties = Vec::new();

            n_bytes += Property::WillDelayInterval(w.delay_interval)
                .encode(&mut properties)
                .await?;
            n_bytes += Property::PayloadFormatIndicator(w.payload_format_indicator)
                .encode(&mut properties)
                .await?;
            if let Some(v) = w.message_expiry_interval {
                n_bytes += Property::MessageExpiryInterval(v)
                    .encode(&mut properties)
                    .await?;
            }
            n_bytes += Property::ContentType(w.content_type)
                .encode(&mut properties)
                .await?;
            if let Some(response_topic) = w.response_topic {
                n_bytes += Property::ResponseTopic(response_topic)
                    .encode(&mut properties)
                    .await?;
            }
            if let Some(v) = w.correlation_data {
                n_bytes += Property::CorrelationData(v).encode(&mut properties).await?;
            }
            for (k, v) in w.user_properties {
                n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
            }

            n_bytes += codec::write_variable_byte_integer(properties.len() as u32, &mut writer).await?;
            writer.write_all(&properties).await?;

            n_bytes += codec::write_utf8_string(&w.topic.to_string(), &mut writer).await?;
            n_bytes += codec::write_binary_data(&w.message, &mut writer).await?;
        }

        if let Some(v) = self.user_name {
            n_bytes += codec::write_utf8_string(&v, &mut writer).await?;
        }

        if let Some(v) = self.password {
            n_bytes += codec::write_binary_data(&v, &mut writer).await?;
        }

        Ok(n_bytes)
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(mut reader: R) -> SageResult<Self> {
        let protocol_name = codec::read_utf8_string(&mut reader).await?;
        if protocol_name != "MQTT" {
            return Err(MalformedPacket.into());
        }

        let protocol_version = codec::read_byte(&mut reader).await?;
        if protocol_version != 0x05 {
            return Err(MalformedPacket.into());
        }

        let flags = ConnectFlags::read(&mut reader).await?;

        let clean_start = flags.clean_start;

        let keep_alive = codec::read_two_byte_integer(&mut reader).await?;

        let mut session_expiry_interval = None;
        let mut receive_maximum = DEFAULT_RECEIVE_MAXIMUM;
        let mut maximum_packet_size = None;
        let mut topic_alias_maximum = DEFAULT_TOPIC_ALIAS_MAXIMUM;
        let mut request_response_information = DEFAULT_REQUEST_RESPONSE_INFORMATION;
        let mut request_problem_information = DEFAULT_REQUEST_PROBLEM_INFORMATION;
        let mut user_properties = Vec::new();
        let mut authentication_method = None;
        let mut authentication_data = Default::default();

        let mut decoder = PropertiesDecoder::take(&mut reader).await?;

        while decoder.has_properties() {
            match decoder.read().await? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = Some(v),
                Property::ReceiveMaximum(v) => receive_maximum = v,
                Property::MaximumPacketSize(v) => maximum_packet_size = Some(v),
                Property::TopicAliasMaximum(v) => topic_alias_maximum = v,
                Property::RequestResponseInformation(v) => request_response_information = v,
                Property::RequestProblemInformation(v) => request_problem_information = v,
                Property::AuthenticationMethod(v) => authentication_method = Some(v),
                Property::AuthenticationData(v) => authentication_data = v,
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(ProtocolError.into()),
            };
        }
        let reader = decoder.into_inner();

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

        // Payload
        let client_id = {
            let client_id = codec::read_utf8_string(reader).await?;
            if client_id.is_empty() {
                None
            } else {
                if client_id.len() > 23 || client_id.chars().any(|c| !('0'..='z').contains(&c)) {
                    return Err(ClientIdentifierNotValid.into());
                }
                Some(client_id)
            }
        };

        let (reader, will) = if flags.will {
            let mut delay_interval = DEFAULT_WILL_DELAY_INTERVAL;
            let mut payload_format_indicator = DEFAULT_PAYLOAD_FORMAT_INDICATOR;
            let mut message_expiry_interval = None;
            let mut content_type = Default::default();
            let mut response_topic = None;
            let mut correlation_data = None;
            let mut user_properties = Vec::new();

            let mut decoder = PropertiesDecoder::take(reader).await?;
            while decoder.has_properties() {
                match decoder.read().await? {
                    Property::WillDelayInterval(v) => delay_interval = v,
                    Property::PayloadFormatIndicator(v) => payload_format_indicator = v,
                    Property::MessageExpiryInterval(v) => message_expiry_interval = Some(v),
                    Property::ContentType(v) => content_type = v,
                    Property::ResponseTopic(v) => response_topic = Some(v),
                    Property::CorrelationData(v) => correlation_data = Some(v),
                    Property::UserProperty(k, v) => user_properties.push((k, v)),
                    _ => return Err(ProtocolError.into()),
                }
            }
            let reader = decoder.into_inner();
            let topic = Topic::from(codec::read_utf8_string(reader).await?);
            let message = codec::read_binary_data(reader).await?;
            (
                reader,
                Some(Will {
                    qos: flags.will_qos,
                    retain: flags.will_retain,
                    delay_interval,
                    payload_format_indicator,
                    message_expiry_interval,
                    content_type,
                    response_topic,
                    correlation_data,
                    user_properties,
                    topic,
                    message,
                }),
            )
        } else {
            (reader, None)
        };

        let user_name = if flags.user_name {
            Some(codec::read_utf8_string(reader).await?)
        } else {
            None
        };

        let password = if flags.password {
            Some(codec::read_binary_data(reader).await?)
        } else {
            None
        };

        Ok(Connect {
            clean_start,
            user_name,
            password,
            keep_alive,
            session_expiry_interval,
            receive_maximum,
            maximum_packet_size,
            topic_alias_maximum,
            request_response_information,
            request_problem_information,
            authentication,
            user_properties,
            client_id,
            will,
        })
    }
}

impl ConnectFlags {
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let bits = ((self.user_name as u8) << 7)
            | ((self.password as u8) << 6)
            | ((self.will_retain as u8) << 5)
            | (self.will_qos as u8) << 3
            | ((self.will as u8) << 2)
            | ((self.clean_start as u8) << 1);
        codec::write_byte(bits, writer).await
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let bits = codec::read_byte(reader).await?;

        if bits & 0x01 != 0 {
            Err(MalformedPacket.into())
        } else {
            Ok(ConnectFlags {
                user_name: (bits & 0b1000_0000) >> 7 > 0,
                password: (bits & 0b0100_0000) >> 6 > 0,
                will_retain: (bits & 0b0010_0000) >> 5 > 0,
                will_qos: ((bits & 0b0001_1000) >> 3).try_into()?,
                will: (bits & 0b0000_0100) >> 2 > 0,
                clean_start: (bits & 0b0000_0010) >> 1 > 0,
            })
        }
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            0, 4, 77, 81, 84, 84, 5, 206, 0, 10, 5, 17, 0, 0, 0, 10, 0, 0, 3, 3, 0, 0, 0, 6, 67,
            108, 111, 90, 101, 101, 0, 6, 79, 114, 101, 103, 111, 110, 0, 6, 87, 105, 108, 108,
            111, 119, 0, 5, 74, 97, 100, 101, 110,
        ]
    }

    fn decoded() -> Connect {
        let keep_alive = 10;
        let session_expiry_interval = Some(10);

        Connect {
            keep_alive,
            clean_start: true,
            session_expiry_interval,
            user_name: Some("Willow".into()),
            password: Some("Jaden".into()),
            will: Some(Will {
                qos: QoS::AtLeastOnce,
                ..Will::with_message(Topic::from("CloZee"), "Oregon")
            }),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn encode_default_auth() {
        let test_data = Connect {
            authentication: Some(Default::default()),
            ..Default::default()
        };
        let mut tested_result = Vec::new();

        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(
            tested_result,
            vec![0, 4, 77, 81, 84, 84, 5, 0, 2, 88, 3, 21, 0, 0, 0, 0]
        );
        assert_eq!(n_bytes, 16);
    }

    #[tokio::test]
    async fn decode_default_auth() {
        let mut test_data = Cursor::new(vec![0, 4, 77, 81, 84, 84, 5, 0, 2, 88, 3, 21, 0, 0, 0, 0]);
        let tested_result = Connect::read(&mut test_data).await.unwrap();
        assert_eq!(
            tested_result,
            Connect {
                authentication: Some(Default::default()),
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();

        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 53);
    }

    #[tokio::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Connect::read(&mut test_data).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
