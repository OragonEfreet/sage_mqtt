use crate::{Error as SageError, ReadByte, Result as SageResult, WriteByte};
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QoS {
    AtMostOnce = 0x00,
    AtLeastOnce = 0x01,
    ExactlyOnce = 0x02,
}

impl ReadByte for QoS {
    fn read_byte<R: Read>(reader: &mut R) -> SageResult<Self> {
        match u8::read_byte(reader)? {
            0x00 => Ok(QoS::AtMostOnce),
            0x01 => Ok(QoS::AtLeastOnce),
            0x02 => Ok(QoS::ExactlyOnce),
            _ => Err(SageError::ProtocolError),
        }
    }
}

impl WriteByte for QoS {
    fn write_byte<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        (self as u8).write_byte(writer)
    }
}

impl TryFrom<u8> for QoS {
    type Error = SageError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(QoS::AtMostOnce),
            0x01 => Ok(QoS::AtLeastOnce),
            0x02 => Ok(QoS::ExactlyOnce),
            _ => Err(Self::Error::MalformedPacket),
        }
    }
}
