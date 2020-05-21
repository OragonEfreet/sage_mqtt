use crate::QoS;

/// The control packet type is present as the first element of the fixed header
/// in an MQTT paquet. It is encoded in a 8bit flag set where the 4 most
/// significant bits represent the type of the paquet and the 4 least are flags
/// where values depend on the type.
#[derive(Debug, Clone, Copy)]
pub enum ControlPacketType {
    RESERVED,
    CONNECT,
    CONNACK,
    PUBLISH {
        duplicate: bool,
        qos: QoS,
        retain: bool,
    },
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}

enum PayloadRequirements {
    None,
    Required,
    Optional,
}

impl From<ControlPacketType> for PayloadRequirements {
    fn from(value: ControlPacketType) -> Self {
        match value {
            ControlPacketType::PUBLISH { .. } => PayloadRequirements::Optional,
            ControlPacketType::CONNECT
            | ControlPacketType::SUBSCRIBE
            | ControlPacketType::SUBACK
            | ControlPacketType::UNSUBSCRIBE
            | ControlPacketType::UNSUBACK => PayloadRequirements::Required,
            _ => PayloadRequirements::None,
        }
    }
}
