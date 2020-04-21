use crate::{
    Connack, Connect, ControlPacketType, Decode, Encode, Error, FixedHeader, Publish,
    Result as SageResult,
};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum ControlPacket {
    Connect(Connect),
    Connack(Connack),
    Publish(Publish),
}

impl Encode for ControlPacket {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut variable_and_payload = Vec::new();

        let (packet_type, remaining_size) = match self {
            ControlPacket::Connect(packet) => (
                ControlPacketType::CONNECT,
                packet.write(&mut variable_and_payload)? as u32,
            ),
            ControlPacket::Connack(packet) => (
                ControlPacketType::CONNACK,
                packet.write(&mut variable_and_payload)? as u32,
            ),
            ControlPacket::Publish(packet) => (
                ControlPacketType::PUBLISH {
                    duplicate: packet.duplicate,
                    qos: packet.qos,
                    retain: packet.retain,
                },
                packet.write(&mut variable_and_payload)? as u32,
            ),
        };

        let fixed_size = FixedHeader {
            packet_type,
            remaining_size,
        }
        .encode(writer)?;

        writer.write_all(&variable_and_payload)?;
        Ok(fixed_size)
    }
}

impl Decode for ControlPacket {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let fixed_header = FixedHeader::decode(reader)?;

        let packet = match fixed_header.packet_type {
            ControlPacketType::CONNECT => ControlPacket::Connect(Connect::read(reader)?),
            ControlPacketType::CONNACK => ControlPacket::Connack(Connack::read(reader)?),
            ControlPacketType::PUBLISH {
                duplicate,
                qos,
                retain,
            } => ControlPacket::Publish(Publish::read(
                reader,
                duplicate,
                qos,
                retain,
                fixed_header.remaining_size,
            )?),
            _ => return Err(Error::ProtocolError),
        };

        Ok(packet)
    }
}
