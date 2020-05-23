use crate::{codec, ControlPacketType, Result as SageResult};
use async_std::io::{Read, Write};
use std::marker::Unpin;

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl FixedHeader {
    pub async fn encode<W: Write + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = codec::write_control_packet_type(self.packet_type, writer).await?;
        n += codec::write_variable_byte_integer(self.remaining_size as u32, writer).await?;
        Ok(n)
    }

    pub async fn decode<R: Read + Unpin>(reader: &mut R) -> SageResult<Self> {
        let packet_type = codec::read_control_packet_type(reader).await?;
        let remaining_size = codec::read_variable_byte_integer(reader).await? as usize;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
