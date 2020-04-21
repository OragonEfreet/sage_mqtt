use crate::Error as SageError;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QoS {
    AtMostOnce = 0x00,
    AtLeastOnce = 0x01,
    ExactlyOnce = 0x02,
}

impl Default for QoS {
    fn default() -> Self {
        QoS::AtMostOnce
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
