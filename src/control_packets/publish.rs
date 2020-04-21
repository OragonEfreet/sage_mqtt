use crate::{Decode, Encode, QoS, Result as SageResult, UTF8String};

use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Publish {
    topic_name: String,
    packet_identifier: Option<u16>,
}

impl Default for Publish {
    fn default() -> Self {
        Publish {
            topic_name: Default::default(),
            packet_identifier: None,
        }
    }
}

impl Publish {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let n_bytes = UTF8String(self.topic_name).encode(writer)?;

        // Packet identifier

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R, qos: QoS) -> SageResult<Self> {
        let topic_name = UTF8String::decode(reader)?.into();

        let packet_identifier = None;

        Ok(Publish {
            topic_name,
            packet_identifier,
        })
    }
}
