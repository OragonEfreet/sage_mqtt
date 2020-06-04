use crate::QoS;

/// The control packet type is present as the first element of the fixed header
/// in an MQTT paquet. It is encoded in a 8bit flag set where the 4 most
/// significant bits represent the type of the paquet and the 4 least are flags
/// where values depend on the type.
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
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

impl From<PacketType> for PayloadRequirements {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::PUBLISH { .. } => PayloadRequirements::Optional,
            PacketType::CONNECT
            | PacketType::SUBSCRIBE
            | PacketType::SUBACK
            | PacketType::UNSUBSCRIBE
            | PacketType::UNSUBACK => PayloadRequirements::Required,
            _ => PayloadRequirements::None,
        }
    }
}
