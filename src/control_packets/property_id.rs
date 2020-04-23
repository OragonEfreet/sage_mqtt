use crate::{Error, ReadVariableByteInteger, Result as SageResult, WriteVariableByteInteger};
use std::io::{Read, Write};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PropertyId {
    PayloadFormatIndicator = 0x01,
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0B,
    SessionExpiryInterval = 0x11,
    AssignedClientIdentifier = 0x12,
    ServerKeepAlive = 0x13,
    AuthenticationMethod = 0x15,
    AuthenticationData = 0x16,
    RequestProblemInformation = 0x17,
    WillDelayInterval = 0x18,
    RequestResponseInformation = 0x19,
    ResponseInformation = 0x1A,
    ServerReference = 0x1C,
    ReasonString = 0x1F,
    ReceiveMaximum = 0x21,
    TopicAliasMaximum = 0x22,
    TopicAlias = 0x23,
    MaximumQoS = 0x24,
    RetainAvailable = 0x25,
    UserProperty = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2A,
}

impl WriteVariableByteInteger for PropertyId {
    fn write_variable_byte_integer<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        (self as u32).write_variable_byte_integer(writer)
    }
}

impl ReadVariableByteInteger for PropertyId {
    fn read_variable_byte_integer<R: Read>(reader: &mut R) -> SageResult<Self> {
        match u32::read_variable_byte_integer(reader)? {
            0x01 => Ok(PropertyId::PayloadFormatIndicator),
            0x02 => Ok(PropertyId::MessageExpiryInterval),
            0x03 => Ok(PropertyId::ContentType),
            0x08 => Ok(PropertyId::ResponseTopic),
            0x09 => Ok(PropertyId::CorrelationData),
            0x0B => Ok(PropertyId::SubscriptionIdentifier),
            0x11 => Ok(PropertyId::SessionExpiryInterval),
            0x12 => Ok(PropertyId::AssignedClientIdentifier),
            0x13 => Ok(PropertyId::ServerKeepAlive),
            0x15 => Ok(PropertyId::AuthenticationMethod),
            0x16 => Ok(PropertyId::AuthenticationData),
            0x17 => Ok(PropertyId::RequestProblemInformation),
            0x18 => Ok(PropertyId::WillDelayInterval),
            0x19 => Ok(PropertyId::RequestResponseInformation),
            0x1A => Ok(PropertyId::ResponseInformation),
            0x1C => Ok(PropertyId::ServerReference),
            0x1F => Ok(PropertyId::ReasonString),
            0x21 => Ok(PropertyId::ReceiveMaximum),
            0x22 => Ok(PropertyId::TopicAliasMaximum),
            0x23 => Ok(PropertyId::TopicAlias),
            0x24 => Ok(PropertyId::MaximumQoS),
            0x25 => Ok(PropertyId::RetainAvailable),
            0x26 => Ok(PropertyId::UserProperty),
            0x27 => Ok(PropertyId::MaximumPacketSize),
            0x28 => Ok(PropertyId::WildcardSubscriptionAvailable),
            0x29 => Ok(PropertyId::SubscriptionIdentifierAvailable),
            0x2A => Ok(PropertyId::SharedSubscriptionAvailable),
            _ => Err(Error::ProtocolError),
        }
    }
}
