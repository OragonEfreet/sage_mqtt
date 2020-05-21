use crate::{codec, Error as SageError, QoS, Result as SageResult};
use std::io::{Read, Write};

pub fn write_qos<W: Write>(qos: QoS, writer: &mut W) -> SageResult<usize> {
    codec::write_byte(qos as u8, writer)
}

pub fn read_qos<R: Read>(reader: &mut R) -> SageResult<QoS> {
    match codec::read_byte(reader)? {
        0x00 => Ok(QoS::AtMostOnce),
        0x01 => Ok(QoS::AtLeastOnce),
        0x02 => Ok(QoS::ExactlyOnce),
        _ => Err(SageError::ProtocolError),
    }
}
