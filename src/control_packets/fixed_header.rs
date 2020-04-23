use crate::{
    ControlPacketType, ReadByte, ReadVariableByteInteger, Result as SageResult, WriteByte,
    WriteVariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl FixedHeader {
    pub fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = self.packet_type.write_byte(writer)?;
        n += self.remaining_size.write_variable_byte_integer(writer)?;
        Ok(n)
    }

    pub fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type = ControlPacketType::read_byte(reader)?;
        let remaining_size = usize::read_variable_byte_integer(reader)?;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
