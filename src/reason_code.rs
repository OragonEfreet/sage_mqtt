use crate::{ControlPacketType, Error, Result as SageResult, WriteByte};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReasonCode {
    Success,
    NormalDisconnection,
    GrantedQoS0,
    GrantedQoS1,
    GrantedQoS2,
    DisconnectWithWillMessage,
    NoMatchingSubscribers,
    NoSubscriptionExisted,
    ContinueAuthentication,
    ReAuthenticate,
    UnspecifiedError,
    MalformedPacket,
    ProtocolError,
    ImplementationSpecificError,
    UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    BadUserNameOrPassword,
    NotAuthorized,
    ServerUnavailable,
    ServerBusy,
    Banned,
    ServerShuttingDown,
    BadAuthenticationMethod,
    KeepAliveTimeout,
    SessionTakenOver,
    TopicFilterInvalid,
    TopicNameInvalid,
    PacketIdentifierInUse,
    PacketIdentifierNotFound,
    ReceiveMaximumExceeded,
    TopicAliasInvalid,
    PacketTooLarge,
    MessageRateTooHigh,
    QuotaExceeded,
    AdministrativeAction,
    PayloadFormatInvalid,
    RetainNotSupported,
    QoSNotSupported,
    UseAnotherServer,
    ServerMoved,
    SharedSubscriptionsNotSupported,
    ConnectionRateExceeded,
    MaximumConnectTime,
    SubscriptionIdentifiersNotSupported,
    WildcardSubscriptionsNotSupported,
}

impl ReasonCode {
    pub fn try_parse(code: u8, packet_type: ControlPacketType) -> SageResult<Self> {
        match (code, packet_type) {
            (0x00, ControlPacketType::CONNACK) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::PUBACK) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::PUBREC) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::PUBREL) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::PUBCOMP) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::UNSUBACK) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::AUTH) => Ok(ReasonCode::Success),
            (0x00, ControlPacketType::DISCONNECT) => Ok(ReasonCode::NormalDisconnection),
            (0x00, ControlPacketType::SUBACK) => Ok(ReasonCode::GrantedQoS0),
            (0x01, ControlPacketType::SUBACK) => Ok(ReasonCode::GrantedQoS1),
            (0x02, ControlPacketType::SUBACK) => Ok(ReasonCode::GrantedQoS2),
            (0x04, ControlPacketType::DISCONNECT) => Ok(ReasonCode::DisconnectWithWillMessage),
            (0x10, ControlPacketType::PUBACK) => Ok(ReasonCode::NoMatchingSubscribers),
            (0x10, ControlPacketType::PUBREC) => Ok(ReasonCode::NoMatchingSubscribers),
            (0x11, ControlPacketType::UNSUBACK) => Ok(ReasonCode::NoSubscriptionExisted),
            (0x18, ControlPacketType::AUTH) => Ok(ReasonCode::ContinueAuthentication),
            (0x19, ControlPacketType::AUTH) => Ok(ReasonCode::ReAuthenticate),
            (0x80, ControlPacketType::CONNACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, ControlPacketType::PUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, ControlPacketType::PUBREC) => Ok(ReasonCode::UnspecifiedError),
            (0x80, ControlPacketType::SUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, ControlPacketType::UNSUBACK) => Ok(ReasonCode::UnspecifiedError),
            (0x80, ControlPacketType::DISCONNECT) => Ok(ReasonCode::UnspecifiedError),
            (0x81, ControlPacketType::CONNACK) => Ok(ReasonCode::MalformedPacket),
            (0x81, ControlPacketType::DISCONNECT) => Ok(ReasonCode::MalformedPacket),
            (0x82, ControlPacketType::CONNACK) => Ok(ReasonCode::ProtocolError),
            (0x82, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ProtocolError),
            (0x83, ControlPacketType::CONNACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, ControlPacketType::PUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, ControlPacketType::PUBREC) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, ControlPacketType::SUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, ControlPacketType::UNSUBACK) => Ok(ReasonCode::ImplementationSpecificError),
            (0x83, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ImplementationSpecificError),
            (0x84, ControlPacketType::CONNACK) => Ok(ReasonCode::UnsupportedProtocolVersion),
            (0x85, ControlPacketType::CONNACK) => Ok(ReasonCode::ClientIdentifierNotValid),
            (0x86, ControlPacketType::CONNACK) => Ok(ReasonCode::BadUserNameOrPassword),
            (0x87, ControlPacketType::CONNACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, ControlPacketType::PUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, ControlPacketType::PUBREC) => Ok(ReasonCode::NotAuthorized),
            (0x87, ControlPacketType::SUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, ControlPacketType::UNSUBACK) => Ok(ReasonCode::NotAuthorized),
            (0x87, ControlPacketType::DISCONNECT) => Ok(ReasonCode::NotAuthorized),
            (0x88, ControlPacketType::CONNACK) => Ok(ReasonCode::ServerUnavailable),
            (0x89, ControlPacketType::CONNACK) => Ok(ReasonCode::ServerBusy),
            (0x89, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ServerBusy),
            (0x8A, ControlPacketType::CONNACK) => Ok(ReasonCode::Banned),
            (0x8B, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ServerShuttingDown),
            (0x8C, ControlPacketType::CONNACK) => Ok(ReasonCode::BadAuthenticationMethod),
            (0x8C, ControlPacketType::DISCONNECT) => Ok(ReasonCode::BadAuthenticationMethod),
            (0x8D, ControlPacketType::DISCONNECT) => Ok(ReasonCode::KeepAliveTimeout),
            (0x8E, ControlPacketType::DISCONNECT) => Ok(ReasonCode::SessionTakenOver),
            (0x8F, ControlPacketType::SUBACK) => Ok(ReasonCode::TopicFilterInvalid),
            (0x8F, ControlPacketType::UNSUBACK) => Ok(ReasonCode::TopicFilterInvalid),
            (0x8F, ControlPacketType::DISCONNECT) => Ok(ReasonCode::TopicFilterInvalid),
            (0x90, ControlPacketType::CONNACK) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, ControlPacketType::PUBACK) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, ControlPacketType::PUBREC) => Ok(ReasonCode::TopicNameInvalid),
            (0x90, ControlPacketType::DISCONNECT) => Ok(ReasonCode::TopicNameInvalid),
            (0x91, ControlPacketType::PUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, ControlPacketType::PUBREC) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, ControlPacketType::SUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x91, ControlPacketType::UNSUBACK) => Ok(ReasonCode::PacketIdentifierInUse),
            (0x92, ControlPacketType::PUBREL) => Ok(ReasonCode::PacketIdentifierNotFound),
            (0x92, ControlPacketType::PUBCOMP) => Ok(ReasonCode::PacketIdentifierNotFound),
            (0x93, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ReceiveMaximumExceeded),
            (0x94, ControlPacketType::DISCONNECT) => Ok(ReasonCode::TopicAliasInvalid),
            (0x95, ControlPacketType::CONNACK) => Ok(ReasonCode::PacketTooLarge),
            (0x95, ControlPacketType::DISCONNECT) => Ok(ReasonCode::PacketTooLarge),
            (0x96, ControlPacketType::DISCONNECT) => Ok(ReasonCode::MessageRateTooHigh),
            (0x97, ControlPacketType::CONNACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, ControlPacketType::PUBACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, ControlPacketType::PUBREC) => Ok(ReasonCode::QuotaExceeded),
            (0x97, ControlPacketType::SUBACK) => Ok(ReasonCode::QuotaExceeded),
            (0x97, ControlPacketType::DISCONNECT) => Ok(ReasonCode::QuotaExceeded),
            (0x98, ControlPacketType::DISCONNECT) => Ok(ReasonCode::AdministrativeAction),
            (0x99, ControlPacketType::CONNACK) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, ControlPacketType::PUBACK) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, ControlPacketType::PUBREC) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x99, ControlPacketType::DISCONNECT) => Ok(ReasonCode::PayloadFormatInvalid),
            (0x9A, ControlPacketType::CONNACK) => Ok(ReasonCode::RetainNotSupported),
            (0x9A, ControlPacketType::DISCONNECT) => Ok(ReasonCode::RetainNotSupported),
            (0x9B, ControlPacketType::CONNACK) => Ok(ReasonCode::QoSNotSupported),
            (0x9B, ControlPacketType::DISCONNECT) => Ok(ReasonCode::QoSNotSupported),
            (0x9C, ControlPacketType::CONNACK) => Ok(ReasonCode::UseAnotherServer),
            (0x9C, ControlPacketType::DISCONNECT) => Ok(ReasonCode::UseAnotherServer),
            (0x9D, ControlPacketType::CONNACK) => Ok(ReasonCode::ServerMoved),
            (0x9D, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ServerMoved),
            (0x9E, ControlPacketType::SUBACK) => Ok(ReasonCode::SharedSubscriptionsNotSupported),
            (0x9E, ControlPacketType::DISCONNECT) => {
                Ok(ReasonCode::SharedSubscriptionsNotSupported)
            }
            (0x9F, ControlPacketType::CONNACK) => Ok(ReasonCode::ConnectionRateExceeded),
            (0x9F, ControlPacketType::DISCONNECT) => Ok(ReasonCode::ConnectionRateExceeded),
            (0xA0, ControlPacketType::DISCONNECT) => Ok(ReasonCode::MaximumConnectTime),
            (0xA1, ControlPacketType::SUBACK) => {
                Ok(ReasonCode::SubscriptionIdentifiersNotSupported)
            }
            (0xA1, ControlPacketType::DISCONNECT) => {
                Ok(ReasonCode::SubscriptionIdentifiersNotSupported)
            }
            (0xA2, ControlPacketType::SUBACK) => Ok(ReasonCode::WildcardSubscriptionsNotSupported),
            (0xA2, ControlPacketType::DISCONNECT) => {
                Ok(ReasonCode::WildcardSubscriptionsNotSupported)
            }
            _ => Err(Error::ProtocolError),
        }
    }
}

impl WriteByte for ReasonCode {
    fn write_byte<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        match self {
            ReasonCode::Success | ReasonCode::NormalDisconnection | ReasonCode::GrantedQoS0 => 0x00,
            ReasonCode::GrantedQoS1 => 0x01,
            ReasonCode::GrantedQoS2 => 0x02,
            ReasonCode::DisconnectWithWillMessage => 0x04,
            ReasonCode::NoMatchingSubscribers => 0x10,
            ReasonCode::NoSubscriptionExisted => 0x11,
            ReasonCode::ContinueAuthentication => 0x18,
            ReasonCode::ReAuthenticate => 0x19,
            ReasonCode::UnspecifiedError => 0x80,
            ReasonCode::MalformedPacket => 0x81,
            ReasonCode::ProtocolError => 0x82,
            ReasonCode::ImplementationSpecificError => 0x83,
            ReasonCode::UnsupportedProtocolVersion => 0x84,
            ReasonCode::ClientIdentifierNotValid => 0x85,
            ReasonCode::BadUserNameOrPassword => 0x86,
            ReasonCode::NotAuthorized => 0x87,
            ReasonCode::ServerUnavailable => 0x88,
            ReasonCode::ServerBusy => 0x89,
            ReasonCode::Banned => 0x8A,
            ReasonCode::ServerShuttingDown => 0x8B,
            ReasonCode::BadAuthenticationMethod => 0x8C,
            ReasonCode::KeepAliveTimeout => 0x8D,
            ReasonCode::SessionTakenOver => 0x8E,
            ReasonCode::TopicFilterInvalid => 0x8F,
            ReasonCode::TopicNameInvalid => 0x90,
            ReasonCode::PacketIdentifierInUse => 0x91,
            ReasonCode::PacketIdentifierNotFound => 0x92,
            ReasonCode::ReceiveMaximumExceeded => 0x93,
            ReasonCode::TopicAliasInvalid => 0x94,
            ReasonCode::PacketTooLarge => 0x95,
            ReasonCode::MessageRateTooHigh => 0x96,
            ReasonCode::QuotaExceeded => 0x97,
            ReasonCode::AdministrativeAction => 0x98,
            ReasonCode::PayloadFormatInvalid => 0x99,
            ReasonCode::RetainNotSupported => 0x9A,
            ReasonCode::QoSNotSupported => 0x9B,
            ReasonCode::UseAnotherServer => 0x9C,
            ReasonCode::ServerMoved => 0x9D,
            ReasonCode::SharedSubscriptionsNotSupported => 0x9E,
            ReasonCode::ConnectionRateExceeded => 0x9F,
            ReasonCode::MaximumConnectTime => 0xA0,
            ReasonCode::SubscriptionIdentifiersNotSupported => 0xA1,
            ReasonCode::WildcardSubscriptionsNotSupported => 0xA2,
        }
        .write_byte(writer)
    }
}
