use crate::{
    ControlPacketType, Decode, Encode, ReadVariableByteInteger, Result as SageResult,
    WriteVariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl Encode for FixedHeader {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = self.packet_type.encode(writer)?;
        n += self.remaining_size.write_variable_byte_integer(writer)?;
        Ok(n)
    }
}

impl Decode for FixedHeader {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type = ControlPacketType::decode(reader)?;
        let remaining_size = usize::read_variable_byte_integer(reader)?;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
