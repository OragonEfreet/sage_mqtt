use crate::{codec, ControlPacketType, Result as SageResult};
use futures::io::{AsyncRead, AsyncWrite};
use std::marker::Unpin;

#[derive(Debug)]
pub struct FixedHeader {
    pub packet_type: ControlPacketType,
    pub remaining_size: usize,
}

impl FixedHeader {
    pub async fn encode<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n = codec::write_control_packet_type(self.packet_type, writer).await?;
        n += codec::write_variable_byte_integer(self.remaining_size as u32, writer).await?;
        Ok(n)
    }

    pub async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let packet_type = codec::read_control_packet_type(reader).await?;
        let remaining_size = codec::read_variable_byte_integer(reader).await? as usize;
        Ok(FixedHeader {
            packet_type,
            remaining_size,
        })
    }
}
