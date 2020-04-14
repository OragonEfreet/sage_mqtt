use crate::{ControlPacketType, VariableByteInteger};

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
    Authenticationmethod = 0x15,
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
    MaximumPackerSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2A,
}

impl From<VariableByteInteger> for Option<PropertyId> {
    fn from(value: VariableByteInteger) -> Self {
        match value {
            VariableByteInteger(0x01) => Some(PropertyId::PayloadFormatIndicator),
            VariableByteInteger(0x02) => Some(PropertyId::MessageExpiryInterval),
            VariableByteInteger(0x03) => Some(PropertyId::ContentType),
            VariableByteInteger(0x08) => Some(PropertyId::ResponseTopic),
            VariableByteInteger(0x09) => Some(PropertyId::CorrelationData),
            VariableByteInteger(0x0B) => Some(PropertyId::SubscriptionIdentifier),
            VariableByteInteger(0x11) => Some(PropertyId::SessionExpiryInterval),
            VariableByteInteger(0x12) => Some(PropertyId::AssignedClientIdentifier),
            VariableByteInteger(0x13) => Some(PropertyId::ServerKeepAlive),
            VariableByteInteger(0x15) => Some(PropertyId::Authenticationmethod),
            VariableByteInteger(0x16) => Some(PropertyId::AuthenticationData),
            VariableByteInteger(0x17) => Some(PropertyId::RequestProblemInformation),
            VariableByteInteger(0x18) => Some(PropertyId::WillDelayInterval),
            VariableByteInteger(0x19) => Some(PropertyId::RequestResponseInformation),
            VariableByteInteger(0x1A) => Some(PropertyId::ResponseInformation),
            VariableByteInteger(0x1C) => Some(PropertyId::ServerReference),
            VariableByteInteger(0x1F) => Some(PropertyId::ReasonString),
            VariableByteInteger(0x21) => Some(PropertyId::ReceiveMaximum),
            VariableByteInteger(0x22) => Some(PropertyId::TopicAliasMaximum),
            VariableByteInteger(0x23) => Some(PropertyId::TopicAlias),
            VariableByteInteger(0x24) => Some(PropertyId::MaximumQoS),
            VariableByteInteger(0x25) => Some(PropertyId::RetainAvailable),
            VariableByteInteger(0x26) => Some(PropertyId::UserProperty),
            VariableByteInteger(0x27) => Some(PropertyId::MaximumPackerSize),
            VariableByteInteger(0x28) => Some(PropertyId::WildcardSubscriptionAvailable),
            VariableByteInteger(0x29) => Some(PropertyId::SubscriptionIdentifierAvailable),
            VariableByteInteger(0x2A) => Some(PropertyId::SharedSubscriptionAvailable),
            _ => None,
        }
    }
}

impl PropertyId {
    pub fn allowed(&self, packet_type: &ControlPacketType) -> bool {
        match (self, packet_type) {
            (PropertyId::PayloadFormatIndicator, ControlPacketType::PUBLISH { .. })
            | (PropertyId::MessageExpiryInterval, ControlPacketType::PUBLISH { .. })
            | (PropertyId::ContentType, ControlPacketType::PUBLISH { .. })
            | (PropertyId::ResponseTopic, ControlPacketType::PUBLISH { .. })
            | (PropertyId::CorrelationData, ControlPacketType::PUBLISH { .. })
            | (PropertyId::SubscriptionIdentifier, ControlPacketType::PUBLISH { .. })
            | (PropertyId::SubscriptionIdentifier, ControlPacketType::SUBSCRIBE)
            | (PropertyId::SessionExpiryInterval, ControlPacketType::CONNECT)
            | (PropertyId::SessionExpiryInterval, ControlPacketType::CONNACK)
            | (PropertyId::SessionExpiryInterval, ControlPacketType::DISCONNECT)
            | (PropertyId::AssignedClientIdentifier, ControlPacketType::CONNACK)
            | (PropertyId::ServerKeepAlive, ControlPacketType::CONNACK)
            | (PropertyId::Authenticationmethod, ControlPacketType::CONNECT)
            | (PropertyId::Authenticationmethod, ControlPacketType::CONNACK)
            | (PropertyId::Authenticationmethod, ControlPacketType::AUTH)
            | (PropertyId::AuthenticationData, ControlPacketType::CONNECT)
            | (PropertyId::AuthenticationData, ControlPacketType::CONNACK)
            | (PropertyId::AuthenticationData, ControlPacketType::AUTH)
            | (PropertyId::RequestProblemInformation, ControlPacketType::CONNECT)
            | (PropertyId::RequestResponseInformation, ControlPacketType::CONNECT)
            | (PropertyId::ResponseInformation, ControlPacketType::CONNACK)
            | (PropertyId::ServerReference, ControlPacketType::CONNACK)
            | (PropertyId::ServerReference, ControlPacketType::DISCONNECT)
            | (PropertyId::ReasonString, ControlPacketType::CONNACK)
            | (PropertyId::ReasonString, ControlPacketType::PUBACK)
            | (PropertyId::ReasonString, ControlPacketType::PUBREC)
            | (PropertyId::ReasonString, ControlPacketType::PUBREL)
            | (PropertyId::ReasonString, ControlPacketType::PUBCOMP)
            | (PropertyId::ReasonString, ControlPacketType::SUBACK)
            | (PropertyId::ReasonString, ControlPacketType::UNSUBACK)
            | (PropertyId::ReasonString, ControlPacketType::DISCONNECT)
            | (PropertyId::ReasonString, ControlPacketType::AUTH)
            | (PropertyId::ReceiveMaximum, ControlPacketType::CONNECT)
            | (PropertyId::ReceiveMaximum, ControlPacketType::CONNACK)
            | (PropertyId::TopicAliasMaximum, ControlPacketType::CONNECT)
            | (PropertyId::TopicAliasMaximum, ControlPacketType::CONNACK)
            | (PropertyId::TopicAlias, ControlPacketType::PUBLISH { .. })
            | (PropertyId::MaximumQoS, ControlPacketType::CONNACK)
            | (PropertyId::RetainAvailable, ControlPacketType::CONNACK)
            | (PropertyId::UserProperty, ControlPacketType::CONNECT)
            | (PropertyId::UserProperty, ControlPacketType::CONNACK)
            | (PropertyId::UserProperty, ControlPacketType::PUBLISH { .. })
            | (PropertyId::UserProperty, ControlPacketType::PUBACK)
            | (PropertyId::UserProperty, ControlPacketType::PUBREC)
            | (PropertyId::UserProperty, ControlPacketType::PUBREL)
            | (PropertyId::UserProperty, ControlPacketType::PUBCOMP)
            | (PropertyId::UserProperty, ControlPacketType::SUBSCRIBE)
            | (PropertyId::UserProperty, ControlPacketType::SUBACK)
            | (PropertyId::UserProperty, ControlPacketType::UNSUBSCRIBE)
            | (PropertyId::UserProperty, ControlPacketType::UNSUBACK)
            | (PropertyId::UserProperty, ControlPacketType::DISCONNECT)
            | (PropertyId::UserProperty, ControlPacketType::AUTH)
            | (PropertyId::MaximumPackerSize, ControlPacketType::CONNECT)
            | (PropertyId::MaximumPackerSize, ControlPacketType::CONNACK)
            | (PropertyId::WildcardSubscriptionAvailable, ControlPacketType::CONNACK)
            | (PropertyId::SubscriptionIdentifierAvailable, ControlPacketType::CONNACK)
            | (PropertyId::SharedSubscriptionAvailable, ControlPacketType::CONNACK) => true,
            _ => false,
        }
    }

    pub fn allowed_as_will(&self) -> bool {
        match self {
            _ => true, // 01
                       // 02
                       // 03
                       // 08
                       // 09
                       // 0B
        }
    }
}
