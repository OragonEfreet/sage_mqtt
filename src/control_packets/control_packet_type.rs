use crate::{Bits, Decode, Encode, Error, Result as SageResult};
use std::io::{Read, Write};

/// The control packet type is present as the first element of the fixed header
/// in an MQTT paquet. It is encoded in a 8bit flag set where the 4 most
/// significant bits represent the type of the paquet and the 4 least are flags
/// where values depend on the type.
#[derive(Debug)]
pub enum ControlPacketType {
    RESERVED,
    CONNECT,
    CONNACK,
    PUBLISH {
        duplicate: bool,
        quality_of_service: u8,
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

impl ControlPacketType {
    pub fn needs_packet_identifier(&self) -> bool {
        match *self {
            ControlPacketType::PUBACK
            | ControlPacketType::PUBREC
            | ControlPacketType::PUBREL
            | ControlPacketType::PUBCOMP
            | ControlPacketType::SUBSCRIBE
            | ControlPacketType::SUBACK
            | ControlPacketType::UNSUBSCRIBE
            | ControlPacketType::UNSUBACK => true,
            ControlPacketType::PUBLISH {
                quality_of_service, ..
            } if quality_of_service > 0 => true,
            _ => false,
        }
    }

    pub fn can_have_properties(&self) -> bool {
        match *self {
            ControlPacketType::CONNECT
            | ControlPacketType::CONNACK
            | ControlPacketType::PUBLISH { .. }
            | ControlPacketType::PUBACK
            | ControlPacketType::PUBREC
            | ControlPacketType::PUBREL
            | ControlPacketType::PUBCOMP
            | ControlPacketType::SUBSCRIBE
            | ControlPacketType::SUBACK
            | ControlPacketType::UNSUBSCRIBE
            | ControlPacketType::UNSUBACK
            | ControlPacketType::DISCONNECT
            | ControlPacketType::AUTH => true,
            _ => false,
        }
    }
}

impl Encode for ControlPacketType {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        let packet_type = Bits::from(match &self {
            ControlPacketType::RESERVED => 0b0000_0000,
            ControlPacketType::CONNECT => 0b0001_0000,
            ControlPacketType::CONNACK => 0b0010_0000,
            ControlPacketType::PUBLISH {
                duplicate,
                quality_of_service,
                retain,
            } => 0b0011_0000 | (*duplicate as u8) << 3 | *quality_of_service << 2 | *retain as u8,
            ControlPacketType::PUBACK => 0b0100_0000,
            ControlPacketType::PUBREC => 0b0101_0000,
            ControlPacketType::PUBREL => 0b0110_0010,
            ControlPacketType::PUBCOMP => 0b0111_0000,
            ControlPacketType::SUBSCRIBE => 0b1000_0010,
            ControlPacketType::SUBACK => 0b1001_0000,
            ControlPacketType::UNSUBSCRIBE => 0b1010_0010,
            ControlPacketType::UNSUBACK => 0b1011_0000,
            ControlPacketType::PINGREQ => 0b1100_0000,
            ControlPacketType::PINGRESP => 0b1101_0000,
            ControlPacketType::DISCONNECT => 0b1110_0000,
            ControlPacketType::AUTH => 0b1111_0000,
        });
        Ok(packet_type.encode(writer)?)
    }
}

impl Decode for ControlPacketType {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type: u8 = Bits::decode(reader)?.into();
        let packet_type = match (packet_type >> 4, packet_type & 0b0000_1111) {
            (0b0000, 0b0000) => ControlPacketType::RESERVED,
            (0b0001, 0b0000) => ControlPacketType::CONNECT,
            (0b0010, 0b0000) => ControlPacketType::CONNACK,
            (0b0010, flags) => ControlPacketType::PUBLISH {
                duplicate: (flags & 0b0111) > 0,
                quality_of_service: (flags & 0b0110) >> 1,
                retain: (flags & 0b0001) > 0,
            },
            (0b0100, 0b0000) => ControlPacketType::PUBACK,
            (0b0101, 0b0000) => ControlPacketType::PUBREC,
            (0b0110, 0b0010) => ControlPacketType::PUBREL,
            (0b0111, 0b0000) => ControlPacketType::PUBCOMP,
            (0b1000, 0b0010) => ControlPacketType::SUBSCRIBE,
            (0b1001, 0b0000) => ControlPacketType::SUBACK,
            (0b1010, 0b0010) => ControlPacketType::UNSUBSCRIBE,
            (0b1011, 0b0000) => ControlPacketType::UNSUBACK,
            (0b1100, 0b0000) => ControlPacketType::PINGREQ,
            (0b1101, 0b0000) => ControlPacketType::PINGRESP,
            (0b1110, 0b0000) => ControlPacketType::DISCONNECT,
            (0b1111, 0b0000) => ControlPacketType::AUTH,
            _ => return Err(Error::MalformedPacket),
        };
        Ok(packet_type)
    }
}

#[cfg(test)]
mod unit_control_packet_type {

    use super::*;
    use std::io::Cursor;

    #[test]
    fn mqtt_2_1_3_1() {
        let reserved_flags_per_type = [
            (0b0001, 0b0000),
            (0b0010, 0b0000),
            (0b0100, 0b0000),
            (0b0101, 0b0000),
            (0b0110, 0b0010),
            (0b0111, 0b0000),
            (0b1000, 0b0010),
            (0b1001, 0b0000),
            (0b1010, 0b0010),
            (0b1011, 0b0000),
            (0b1100, 0b0000),
            (0b1101, 0b0000),
            (0b1110, 0b0000),
            (0b1111, 0b0000),
        ];

        for (packet_type, flags) in &reserved_flags_per_type {
            for i in 0b0000..=0b1111 {
                if i == *flags {
                    continue;
                }
                let buffer = [*packet_type, *flags, 0x00];
                let mut test_stream = Cursor::new(buffer);
                assert_matches!(
                    ControlPacketType::decode(&mut test_stream),
                    Err(Error::MalformedPacket)
                );
            }
        }
    }
}
