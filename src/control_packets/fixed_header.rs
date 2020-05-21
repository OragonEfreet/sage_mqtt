use crate::{codec, ControlPacketType, Result as SageResult};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl FixedHeader {
    pub fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = codec::write_control_packet_type(self.packet_type, writer)?;
        n += codec::write_variable_byte_integer(self.remaining_size as u32, writer)?;
        Ok(n)
    }

    pub fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type = codec::read_control_packet_type(reader)?;
        let remaining_size = codec::read_variable_byte_integer(reader)? as usize;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
