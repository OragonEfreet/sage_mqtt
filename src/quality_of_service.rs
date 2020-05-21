use crate::Error as SageError;
use std::convert::TryFrom;

/// Description the quality of service used in message publishing.
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum QoS {
    /// The message is delivered according to the capabilities of the
    /// underlying network. No response is sent by the receiver and no retry is
    /// performed by the sender.
    /// The message arrives at the receiver either once or not at all.
    AtMostOnce = 0x00,

    /// This Quality of Service level ensures that the message arrives at the
    /// receiver at least once. A QoS 1 PUBLISH packet has a Packet Identifier
    /// and is acknowledged by a `Puback` packet.
    AtLeastOnce = 0x01,

    /// This is the highest Quality of Service level, for use when neither loss
    /// nor duplication of messages are acceptable.
    /// There is an increased overhead associated with QoS 2.
    /// A QoS 2 message has a Packet Identifier. The receiver of a QoS 2
    /// `Publish` packet acknowledges receipt with a two-step
    /// acknowledgement process.
    ExactlyOnce = 0x02,
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
