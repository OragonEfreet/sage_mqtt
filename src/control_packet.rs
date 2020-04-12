use crate::{Bits, Decode, Encode, Error, Result as SageResult, VariableByteInteger};
use std::io::{Read, Write};

/// The control packet type is present as the first element of the fixed header
/// in an MQTT paquet. It is encoded in a 8bit flag set where the 4 most
/// significant bits represent the type of the paquet and the 4 least are flags
/// where values depend on the type.
#[derive(Debug)]
pub enum ControlPacketType {
    /// Reserved. Forbidden use.
    RESERVED,

    /// Connection request.
    CONNECT,

    /// Connect acknowledgment
    CONNACK,

    /// Publish message.
    PUBLISH {
        /// Duplicate delivery of the packet.
        duplicate: bool,
        /// Quality of service
        quality_of_service: u8,
        /// Is it a retain message
        retain: bool,
    },
    /// Publish acknowledgment (QoS 1)
    PUBACK,

    /// Publish received (QoS 2 delivery part 1)
    PUBREC,

    /// Publish release (QoS 2 delivery part 2)
    PUBREL,

    /// Publish complete (QoS 2 delivery part 3)
    PUBCOMP,

    /// Subscribe request
    SUBSCRIBE,

    /// Subscribe acknowledgment
    SUBACK,

    /// Unsubscribe request
    UNSUBSCRIBE,

    /// Unsubscribe acknowledgment
    UNSUBACK,

    /// PING request
    PINGREQ,

    /// PING response
    PINGRESP,

    /// Disconnect notification
    DISCONNECT,

    /// Authentication exchange
    AUTH,
}

/// Fixed Header, present in all MQTT Control Packets
#[derive(Debug)]
pub struct FixedHeader {
    /// The packet type.
    pub packet_type: ControlPacketType,

    /// The remaining size of the packet in encoded bytes
    /// including data in the Variable Header and the Payload.
    pub remaining_length: VariableByteInteger,
}

impl Encode for FixedHeader {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        let packet_type = Bits::from(match self.packet_type {
            ControlPacketType::RESERVED => 0b0000_0000,
            ControlPacketType::CONNECT => 0b0001_0000,
            ControlPacketType::CONNACK => 0b0010_0000,
            ControlPacketType::PUBLISH {
                duplicate,
                quality_of_service,
                retain,
            } => 0b0011_0000 | (duplicate as u8) << 3 | quality_of_service << 2 | retain as u8,
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

        let mut n_bytes = packet_type.encode(writer)?;
        n_bytes += self.remaining_length.encode(writer)?;
        Ok(n_bytes)
    }
}

impl Decode for FixedHeader {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type: u8 = Bits::decode(reader)?.into();
        let packet_type = match (packet_type >> 4, packet_type & 0b00001111) {
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

        let remaining_length = VariableByteInteger::decode(reader)?;

        Ok(FixedHeader {
            packet_type,
            remaining_length,
        })
    }
}

/// Variable Header, present in some MQTT Control Packets
#[derive(Debug)]
pub struct VariableHeader;

/// Payload, present in some MQTT Control Packets
#[derive(Debug)]
pub struct Payload;

/// MQTT Control Packet
#[derive(Debug)]
pub struct ControlPacket {
    pub fixed_header: FixedHeader,
    pub variable_header: Option<VariableHeader>,
    pub payload: Option<Payload>,
}
