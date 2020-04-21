use crate::{ControlPacketType, Decode, Encode, Result as SageResult, VariableByteInteger};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl Encode for FixedHeader {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = self.packet_type.encode(writer)?;
        n += VariableByteInteger(self.remaining_size as u32).encode(writer)?;
        Ok(n)
    }
}

impl Decode for FixedHeader {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_type = ControlPacketType::decode(reader)?;
        let remaining_size = u32::from(VariableByteInteger::decode(reader)?) as usize;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
