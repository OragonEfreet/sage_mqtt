use crate::{
    ControlPacketType, Decode, Encode, Error, Property, Result as SageResult, TwoByteInteger,
    VariableByteInteger,
};
use std::io::{Read, Write};

/// MQTT Control Packet
#[derive(Debug)]
pub struct ControlPacket {
    packet_identifier: Option<u16>,
    packet_type: ControlPacketType,
    properties: Vec<Property>,
}

impl Encode for ControlPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_type.encode(writer)?;

        let mut remaining_buffer: Vec<u8> = Vec::new();

        if self.packet_type.needs_packet_identifier() {
            if let Some(packet_identifier) = self.packet_identifier {
                TwoByteInteger(packet_identifier).encode(&mut remaining_buffer)?;
            } else {
                return Err(Error::MalformedPacket);
            }
        }

        n_bytes += self.properties.encode(writer)?;

        n_bytes += VariableByteInteger(remaining_buffer.len() as u32).encode(writer)?;
        n_bytes += remaining_buffer.len();
        writer.write_all(&remaining_buffer)?;

        Ok(n_bytes)
    }
}

impl Decode for ControlPacket {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type = ControlPacketType::decode(reader)?;

        let _remaining_length = VariableByteInteger::decode(reader)?;

        let packet_identifier = if packet_type.needs_packet_identifier() {
            Some(TwoByteInteger::decode(reader)?.into())
        } else {
            None
        };

        let properties = {
            if packet_type.can_have_properties() {
                Decode::decode(reader)?
            } else {
                Default::default()
            }
        };

        Ok(ControlPacket {
            packet_identifier,
            packet_type,
            properties,
        })
    }
}
