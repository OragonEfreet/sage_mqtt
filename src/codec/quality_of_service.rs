use crate::{Byte, Decode, Encode, Error, Result as SageResult};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QoS {
    AtMostOnce = 0x00,
    AtLeastOnce = 0x01,
    ExactlyOnce = 0x02,
}

impl Decode for QoS {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        match Byte::decode(reader)?.into() {
            0x00 => Ok(QoS::AtMostOnce),
            0x01 => Ok(QoS::AtLeastOnce),
            0x02 => Ok(QoS::ExactlyOnce),
            _ => Err(Error::ProtocolError),
        }
    }
}

impl Encode for QoS {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Byte(self as u8).encode(writer)
    }
}
