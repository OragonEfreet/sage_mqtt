use crate::{Error, PacketType, Result as SageResult};
use futures::io::ErrorKind;

/// A `ReasonCode` is an identifier describing a response in any ackowledgement
/// packet (such as `Connack` or `SubAck`)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReasonCode {
    /// Generic success reason code indicating an operation performed well.
    Success,

    /// Used within a `Disconnect` packet to indicate the connection is normal
    /// and the Last Will message should not be sent.
    NormalDisconnection,

    /// The subscription is accepted and the maximum QoS sent will be QoS 0.
    GrantedQoS0,

    /// The subscription is accepted and the maximum QoS sent will be QoS 1.
    GrantedQoS1,

    /// The subscription is accepted and any received QoS will be sent..
    GrantedQoS2,

    /// The client disconnects but want the last will to be sent anyways.
    DisconnectWithWillMessage,

    /// The message is accepted but there are no subscribers.
    NoMatchingSubscribers,

    /// No matching topic filter is being used by the client
    NoSubscriptionExisted,

    /// Continue de authentication with another step
    ContinueAuthentication,

    /// Initiate re-Authentication
    ReAuthenticate,

    /// The server doesn't want or cannot describe the error
    UnspecifiedError,

    /// The control packet cannot be parsed or is ill-formed
    MalformedPacket,

    /// The control packet is well formed but invalid according to specifications.
    ProtocolError,

    /// The operation is valid but not accepted by, the server
    ImplementationSpecificError,

    /// The requested MQTT version is not supported
    UnsupportedProtocolVersion,

    /// The client identifier is not valid
    ClientIdentifierNotValid,

    /// The server does not accept the given user name or password
    BadUserNameOrPassword,

    /// The operation is not permitted
    NotAuthorized,

    /// The server is not available
    ServerUnavailable,

    /// The server is busy
    ServerBusy,

    /// The client is banned
    Banned,

    /// The server is currently shutting down
    ServerShuttingDown,

    /// The authentication method is not supported by the server
    BadAuthenticationMethod,

    /// The connection is closed because not packet has been received for
    /// 1.5 times the keep alive period.
    KeepAliveTimeout,

    /// Another connection using the same client id has connected, causing this
    /// connection to be closed.
    SessionTakenOver,

    /// The topic filter is correctly formed but not accepted by the server.
    TopicFilterInvalid,

    /// The topic name is correctly formed but not accepted by the server
    TopicNameInvalid,

    /// The packet identifier is already in use. This might indicate a mismatch
    /// in the session state between the client and the server.
    PacketIdentifierInUse,

    /// The Packet Identifier is not known. This is not an error during
    /// recovery, but at other times indicates a mismatch between the Session
    /// State on the Client and Server.
    PacketIdentifierNotFound,

    /// The Client or Server has received more than receive maximum.
    ReceiveMaximumExceeded,

    /// The topic alias is invalid
    TopicAliasInvalid,

    /// The packet size is greater than the maximum packet size fo rthis client
    /// or server.
    PacketTooLarge,

    /// The received data is too high
    MessageRateTooHigh,

    /// An implementation or administrative limite has been exceeded
    QuotaExceeded,

    /// The connection is closed due to an administrative action
    AdministrativeAction,

    /// The payload format does not match the one indicated in the payload
    /// format indicator.
    PayloadFormatInvalid,

    /// The server does not support retain messages
    RetainNotSupported,

    /// The client specified a QoS greater than the maximum indicated in the
    /// `Connack` packet.
    QoSNotSupported,

    /// The client should temporarily change its server.
    UseAnotherServer,

    /// The client should permanently change its server.
    ServerMoved,

    /// The server does not support shared subscriptions
    SharedSubscriptionsNotSupported,

    /// The connection is closed because the connection rate is too high
    ConnectionRateExceeded,

    /// The maximum connect time authorized for this connection has exceeded.
    MaximumConnectTime,

    /// The server does no support subscription identifiers.
    SubscriptionIdentifiersNotSupported,

    /// The server does not support wildcard subcriptions.
    WildcardSubscriptionsNotSupported,
}

impl From<Error> for ReasonCode {
    fn from(e: Error) -> Self {
        match e {
            Error::Reason(rc) => rc,
            Error::Io(e) => match e.kind() {
                ErrorKind::UnexpectedEof => ReasonCode::ProtocolError,
                _ => ReasonCode::MalformedPacket,
            },
        }
    }
}

impl ReasonCode {
    pub(crate) fn try_parse(code: u8, packet_type: PacketType) -> SageResult<Self> {
        match (code, packet_type) {
            (0x00, PacketType::CONNACK) => Ok(ReasonCode::Success),
            (0x00, PacketType::PUBACK) => Ok(ReasonCode::Success),
            (0x00, PacketType::PUBREC) => Ok(ReasonCode::Success),
            (0x00, PacketType::PUBREL) => Ok(ReasonCode::Success),
            (0x00, PacketType::PUBCOMP) => Ok(ReasonCode::Success),
            (0x00, PacketType::UNSUBACK) => Ok(ReasonCode::Success),
            (0x00, PacketType::AUTH) => Ok(ReasonCode::Success),
            (0x00, PacketType::DISCONNECT) => Ok(ReasonCode::NormalDisconnection),
            (0x00, PacketType::SUBACK) => Ok(ReasonCode::GrantedQoS0),

            (0x01, PacketType::SUBACK) => Ok(ReasonCode::GrantedQoS1),
            (0x02, PacketType::SUBACK) => Ok(ReasonCode::GrantedQoS2),
            (0x04, PacketType::DISCONNECT) => Ok(ReasonCode::DisconnectWithWillMessage),
            (0x10, PacketType::PUBACK) => Ok(ReasonCode::NoMatchingSubscribers),
            (0x10, PacketType::PUBREC) => Ok(ReasonCode::NoMatchingSubscribers),
            (0x11, PacketType::UNSUBACK) => Ok(ReasonCode::NoSubscriptionExisted),
            (0x18, PacketType::AUTH) => Ok(ReasonCode::ContinueAuthentication),
            (0x19, PacketType::AUTH) => Ok(ReasonCode::ReAuthenticate),
            (0x80, PacketType::CONNACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, PacketType::PUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, PacketType::PUBREC) => Ok(ReasonCode::UnspecifiedError),
            (0x80, PacketType::SUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, PacketType::UNSUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, PacketType::DISCONNECT) => Ok(ReasonCode::UnspecifiedError),
            (0x81, PacketType::CONNACK) => Ok(ReasonCode::MalformedPacket),
            (0x81, PacketType::DISCONNECT) => Ok(ReasonCode::MalformedPacket),
            (0x82, PacketType::CONNACK) => Ok(ReasonCode::ProtocolError),
            (0x82, PacketType::DISCONNECT) => Ok(ReasonCode::ProtocolError),
            (0x83, PacketType::CONNACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, PacketType::PUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, PacketType::PUBREC) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, PacketType::SUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, PacketType::UNSUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, PacketType::DISCONNECT) => Ok(ReasonCode::ImplementationSpecificError),
            (0x84, PacketType::CONNACK) => Ok(ReasonCode::UnsupportedProtocolVersion),
            (0x85, PacketType::CONNACK) => Ok(ReasonCode::ClientIdentifierNotValid),
            (0x86, PacketType::CONNACK) => Ok(ReasonCode::BadUserNameOrPassword),
            (0x87, PacketType::CONNACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, PacketType::PUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, PacketType::PUBREC) => Ok(ReasonCode::NotAuthorized),
            (0x87, PacketType::SUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, PacketType::UNSUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, PacketType::DISCONNECT) => Ok(ReasonCode::NotAuthorized),
            (0x88, PacketType::CONNACK) => Ok(ReasonCode::ServerUnavailable),
            (0x89, PacketType::CONNACK) => Ok(ReasonCode::ServerBusy),
            (0x89, PacketType::DISCONNECT) => Ok(ReasonCode::ServerBusy),
            (0x8A, PacketType::CONNACK) => Ok(ReasonCode::Banned),
            (0x8B, PacketType::DISCONNECT) => Ok(ReasonCode::ServerShuttingDown),
            (0x8C, PacketType::CONNACK) => Ok(ReasonCode::BadAuthenticationMethod),
            (0x8C, PacketType::DISCONNECT) => Ok(ReasonCode::BadAuthenticationMethod),
            (0x8D, PacketType::DISCONNECT) => Ok(ReasonCode::KeepAliveTimeout),
            (0x8E, PacketType::DISCONNECT) => Ok(ReasonCode::SessionTakenOver),
            (0x8F, PacketType::SUBACK) => Ok(ReasonCode::TopicFilterInvalid),
            (0x8F, PacketType::UNSUBACK) => Ok(ReasonCode::TopicFilterInvalid),
            (0x8F, PacketType::DISCONNECT) => Ok(ReasonCode::TopicFilterInvalid),
            (0x90, PacketType::CONNACK) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, PacketType::PUBACK) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, PacketType::PUBREC) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, PacketType::DISCONNECT) => Ok(ReasonCode::TopicNameInvalid),
            (0x91, PacketType::PUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, PacketType::PUBREC) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, PacketType::SUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, PacketType::UNSUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x92, PacketType::PUBREL) => Ok(ReasonCode::PacketIdentifierNotFound),
            (0x92, PacketType::PUBCOMP) => Ok(ReasonCode::PacketIdentifierNotFound),
            (0x93, PacketType::DISCONNECT) => Ok(ReasonCode::ReceiveMaximumExceeded),
            (0x94, PacketType::DISCONNECT) => Ok(ReasonCode::TopicAliasInvalid),
            (0x95, PacketType::CONNACK) => Ok(ReasonCode::PacketTooLarge),
            (0x95, PacketType::DISCONNECT) => Ok(ReasonCode::PacketTooLarge),
            (0x96, PacketType::DISCONNECT) => Ok(ReasonCode::MessageRateTooHigh),
            (0x97, PacketType::CONNACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, PacketType::PUBACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, PacketType::PUBREC) => Ok(ReasonCode::QuotaExceeded),
            (0x97, PacketType::SUBACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, PacketType::DISCONNECT) => Ok(ReasonCode::QuotaExceeded),
            (0x98, PacketType::DISCONNECT) => Ok(ReasonCode::AdministrativeAction),
            (0x99, PacketType::CONNACK) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, PacketType::PUBACK) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, PacketType::PUBREC) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, PacketType::DISCONNECT) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x9A, PacketType::CONNACK) => Ok(ReasonCode::RetainNotSupported),
            (0x9A, PacketType::DISCONNECT) => Ok(ReasonCode::RetainNotSupported),
            (0x9B, PacketType::CONNACK) => Ok(ReasonCode::QoSNotSupported),
            (0x9B, PacketType::DISCONNECT) => Ok(ReasonCode::QoSNotSupported),
            (0x9C, PacketType::CONNACK) => Ok(ReasonCode::UseAnotherServer),
            (0x9C, PacketType::DISCONNECT) => Ok(ReasonCode::UseAnotherServer),
            (0x9D, PacketType::CONNACK) => Ok(ReasonCode::ServerMoved),
            (0x9D, PacketType::DISCONNECT) => Ok(ReasonCode::ServerMoved),
            (0x9E, PacketType::SUBACK) => Ok(ReasonCode::SharedSubscriptionsNotSupported),
            (0x9E, PacketType::DISCONNECT) => Ok(ReasonCode::SharedSubscriptionsNotSupported),
            (0x9F, PacketType::CONNACK) => Ok(ReasonCode::ConnectionRateExceeded),
            (0x9F, PacketType::DISCONNECT) => Ok(ReasonCode::ConnectionRateExceeded),
            (0xA0, PacketType::DISCONNECT) => Ok(ReasonCode::MaximumConnectTime),
            (0xA1, PacketType::SUBACK) => Ok(ReasonCode::SubscriptionIdentifiersNotSupported),
            (0xA1, PacketType::DISCONNECT) => Ok(ReasonCode::SubscriptionIdentifiersNotSupported),
            (0xA2, PacketType::SUBACK) => Ok(ReasonCode::WildcardSubscriptionsNotSupported),
            (0xA2, PacketType::DISCONNECT) => Ok(ReasonCode::WildcardSubscriptionsNotSupported),
            _ => Err(Error::Reason(ReasonCode::ProtocolError)),
        }
    }
}
