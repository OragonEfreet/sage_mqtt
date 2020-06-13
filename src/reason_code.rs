use crate::Error as SageError;
use futures::io::ErrorKind;
use std::convert::TryFrom;

/// A `ReasonCode` is an identifier describing a response in any ackowledgement
/// packet (such as `Connack` or `SubAck`)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReasonCode {
    /// Generic success reason code indicating an operation performed well.
    /// According to the emmiting packet, the following meanings are applied:
    /// - `Disconnect`: Normal Disconnection
    /// - `SubAck`: Granted QoS 0
    Success = 0x00,

    /// The subscription is accepted and the maximum QoS sent will be QoS 1.
    GrantedQoS1 = 0x01,

    /// The subscription is accepted and any received QoS will be sent..
    GrantedQoS2 = 0x02,

    /// The client disconnects but want the last will to be sent anyways.
    DisconnectWithWillMessage = 0x04,

    /// The message is accepted but there are no subscribers.
    NoMatchingSubscribers = 0x10,

    /// No matching topic filter is being used by the client
    NoSubscriptionExisted = 0x11,

    /// Continue de authentication with another step
    ContinueAuthentication = 0x18,

    /// Initiate re-Authentication
    ReAuthenticate = 0x19,

    /// The server doesn't want or cannot describe the error
    UnspecifiedError = 0x80,

    /// The control packet cannot be parsed or is ill-formed
    MalformedPacket = 0x81,

    /// The control packet is well formed but invalid according to specifications.
    ProtocolError = 0x82,

    /// The operation is valid but not accepted by, the server
    ImplementationSpecificError = 0x83,

    /// The requested MQTT version is not supported
    UnsupportedProtocolVersion = 0x84,

    /// The client identifier is not valid
    ClientIdentifierNotValid = 0x85,

    /// The server does not accept the given user name or password
    BadUserNameOrPassword = 0x86,

    /// The operation is not permitted
    NotAuthorized = 0x87,

    /// The server is not available
    ServerUnavailable = 0x88,

    /// The server is busy
    ServerBusy = 0x89,

    /// The client is banned
    Banned = 0x8A,

    /// The server is currently shutting down
    ServerShuttingDown = 0x8B,

    /// The authentication method is not supported by the server
    BadAuthenticationMethod = 0x8C,

    /// The connection is closed because not packet has been received for
    /// 1.5 times the keep alive period.
    KeepAliveTimeout = 0x8D,

    /// Another connection using the same client id has connected, causing this
    /// connection to be closed.
    SessionTakenOver = 0x8E,

    /// The topic filter is correctly formed but not accepted by the server.
    TopicFilterInvalid = 0x8F,

    /// The topic name is correctly formed but not accepted by the server
    TopicNameInvalid = 0x90,

    /// The packet identifier is already in use. This might indicate a mismatch
    /// in the session state between the client and the server.
    PacketIdentifierInUse = 0x91,

    /// The Packet Identifier is not known. This is not an error during
    /// recovery, but at other times indicates a mismatch between the Session
    /// State on the Client and Server.
    PacketIdentifierNotFound = 0x92,

    /// The Client or Server has received more than receive maximum.
    ReceiveMaximumExceeded = 0x93,

    /// The topic alias is invalid
    TopicAliasInvalid = 0x94,

    /// The packet size is greater than the maximum packet size fo rthis client
    /// or server.
    PacketTooLarge = 0x95,

    /// The received data is too high
    MessageRateTooHigh = 0x96,

    /// An implementation or administrative limite has been exceeded
    QuotaExceeded = 0x97,

    /// The connection is closed due to an administrative action
    AdministrativeAction = 0x98,

    /// The payload format does not match the one indicated in the payload
    /// format indicator.
    PayloadFormatInvalid = 0x99,

    /// The server does not support retain messages
    RetainNotSupported = 0x9A,

    /// The client specified a QoS greater than the maximum indicated in the
    /// `Connack` packet.
    QoSNotSupported = 0x9B,

    /// The client should temporarily change its server.
    UseAnotherServer = 0x9C,

    /// The client should permanently change its server.
    ServerMoved = 0x9D,

    /// The server does not support shared subscriptions
    SharedSubscriptionsNotSupported = 0x9E,

    /// The connection is closed because the connection rate is too high
    ConnectionRateExceeded = 0x9F,

    /// The maximum connect time authorized for this connection has exceeded.
    MaximumConnectTime = 0xA0,

    /// The server does no support subscription identifiers.
    SubscriptionIdentifiersNotSupported = 0xA1,

    /// The server does not support wildcard subcriptions.
    WildcardSubscriptionsNotSupported = 0xA2,
}

impl From<SageError> for ReasonCode {
    fn from(e: SageError) -> Self {
        match e {
            SageError::Reason(rc) => rc,
            SageError::Io(e) => match e.kind() {
                ErrorKind::UnexpectedEof => ReasonCode::ProtocolError,
                _ => ReasonCode::MalformedPacket,
            },
        }
    }
}

impl TryFrom<u8> for ReasonCode {
    type Error = SageError;

    fn try_from(value: u8) -> Result<Self, SageError> {
        match value {
            0x00 => Ok(ReasonCode::Success),
            0x01 => Ok(ReasonCode::GrantedQoS1),
            0x02 => Ok(ReasonCode::GrantedQoS2),
            0x04 => Ok(ReasonCode::DisconnectWithWillMessage),
            0x10 => Ok(ReasonCode::NoMatchingSubscribers),
            0x11 => Ok(ReasonCode::NoSubscriptionExisted),
            0x18 => Ok(ReasonCode::ContinueAuthentication),
            0x19 => Ok(ReasonCode::ReAuthenticate),
            0x80 => Ok(ReasonCode::UnspecifiedError),
            0x81 => Ok(ReasonCode::MalformedPacket),
            0x82 => Ok(ReasonCode::ProtocolError),
            0x83 => Ok(ReasonCode::ImplementationSpecificError),
            0x84 => Ok(ReasonCode::UnsupportedProtocolVersion),
            0x85 => Ok(ReasonCode::ClientIdentifierNotValid),
            0x86 => Ok(ReasonCode::BadUserNameOrPassword),
            0x87 => Ok(ReasonCode::NotAuthorized),
            0x88 => Ok(ReasonCode::ServerUnavailable),
            0x89 => Ok(ReasonCode::ServerBusy),
            0x8A => Ok(ReasonCode::Banned),
            0x8B => Ok(ReasonCode::ServerShuttingDown),
            0x8C => Ok(ReasonCode::BadAuthenticationMethod),
            0x8D => Ok(ReasonCode::KeepAliveTimeout),
            0x8E => Ok(ReasonCode::SessionTakenOver),
            0x8F => Ok(ReasonCode::TopicFilterInvalid),
            0x90 => Ok(ReasonCode::TopicNameInvalid),
            0x91 => Ok(ReasonCode::PacketIdentifierInUse),
            0x92 => Ok(ReasonCode::PacketIdentifierNotFound),
            0x93 => Ok(ReasonCode::ReceiveMaximumExceeded),
            0x94 => Ok(ReasonCode::TopicAliasInvalid),
            0x95 => Ok(ReasonCode::PacketTooLarge),
            0x96 => Ok(ReasonCode::MessageRateTooHigh),
            0x97 => Ok(ReasonCode::QuotaExceeded),
            0x98 => Ok(ReasonCode::AdministrativeAction),
            0x99 => Ok(ReasonCode::PayloadFormatInvalid),
            0x9A => Ok(ReasonCode::RetainNotSupported),
            0x9B => Ok(ReasonCode::QoSNotSupported),
            0x9C => Ok(ReasonCode::UseAnotherServer),
            0x9D => Ok(ReasonCode::ServerMoved),
            0x9E => Ok(ReasonCode::SharedSubscriptionsNotSupported),
            0x9F => Ok(ReasonCode::ConnectionRateExceeded),
            0xA0 => Ok(ReasonCode::MaximumConnectTime),
            0xA1 => Ok(ReasonCode::SubscriptionIdentifiersNotSupported),
            0xA2 => Ok(ReasonCode::WildcardSubscriptionsNotSupported),
            _ => Err(SageError::Reason(ReasonCode::ProtocolError)),
        }
    }
}
