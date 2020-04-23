use crate::{
    ControlPacketType, Error, PropertiesDecoder, Property, ReadByte, ReadTwoByteInteger,
    ReasonCode, Result as SageResult, WriteByte, WriteTwoByteInteger, WriteVariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct PubRel {
    pub packet_identifier: u16,
    pub reason_code: ReasonCode,
    pub reason_string: Option<String>,
    pub user_properties: Vec<(String, String)>,
}

impl Default for PubRel {
    fn default() -> Self {
        PubRel {
            packet_identifier: 0,
            reason_code: ReasonCode::Success,
            reason_string: None,
            user_properties: Default::default(),
        }
    }
}

impl PubRel {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_identifier.write_two_byte_integer(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        if n_bytes == 2 && self.reason_code != ReasonCode::Success {
            Ok(2)
        } else {
            n_bytes += self.reason_code.write_byte(writer)?;
            n_bytes += properties.len().write_variable_byte_integer(writer)?;
            writer.write_all(&properties)?;
            Ok(n_bytes)
        }
    }

    pub fn read<R: Read>(reader: &mut R, shortened: bool) -> SageResult<Self> {
        let packet_identifier = u16::read_two_byte_integer(reader)?;

        let mut pubrel = PubRel {
            packet_identifier,
            ..Default::default()
        };

        if shortened {
            pubrel.reason_code = ReasonCode::Success;
        } else {
            pubrel.reason_code =
                ReasonCode::try_parse(u8::read_byte(reader)?, ControlPacketType::PUBREL)?;

            let mut properties = PropertiesDecoder::take(reader)?;
            while properties.has_properties() {
                match properties.read()? {
                    Property::ReasonString(v) => pubrel.reason_string = Some(v),
                    Property::UserProperty(k, v) => pubrel.user_properties.push((k, v)),
                    _ => return Err(Error::ProtocolError),
                }
            }
        }

        Ok(pubrel)
    }
}
