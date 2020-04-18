use crate::{Connect, ControlPacketType, Decode, Encode, Error, FixedHeader, Result as SageResult};
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum ControlPacket {
    Connect(Connect),
}

impl Encode for ControlPacket {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut variable_and_payload = Vec::new();

        let (packet_type, remaining_size) = match self {
            ControlPacket::Connect(packet) => (
                ControlPacketType::CONNECT,
                packet.encode(&mut variable_and_payload)? as u32,
            ),
        };

        let fixed_size = FixedHeader {
            packet_type,
            remaining_size,
        }
        .encode(writer)?;

        writer.write_all(&mut variable_and_payload)?;
        Ok(fixed_size)
    }
}

impl Decode for ControlPacket {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let fixed_header = FixedHeader::decode(reader)?;

        let packet = match fixed_header.packet_type {
            ControlPacketType::CONNECT => ControlPacket::Connect(Connect::decode(reader)?),
            _ => return Err(Error::ProtocolError),
        };

        Ok(packet)
    }
}
