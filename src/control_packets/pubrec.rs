use crate::{
    Byte, ControlPacketType, Decode, Encode, Error, PropertiesDecoder, Property, ReasonCode,
    Result as SageResult, TwoByteInteger, VariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct PubRec {
    pub packet_identifier: u16,
    pub reason_code: ReasonCode,
    pub reason_string: Option<String>,
    pub user_properties: Vec<(String, String)>,
}

impl Default for PubRec {
    fn default() -> Self {
        PubRec {
            packet_identifier: 0,
            reason_code: ReasonCode::Success,
            reason_string: None,
            user_properties: Default::default(),
        }
    }
}

impl PubRec {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = TwoByteInteger(self.packet_identifier).encode(writer)?;

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
            n_bytes += Byte(self.reason_code as u8).encode(writer)?;
            n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
            writer.write_all(&properties)?;
            Ok(n_bytes)
        }
    }

    pub fn read<R: Read>(reader: &mut R, shortened: bool) -> SageResult<Self> {
        let packet_identifier = TwoByteInteger::decode(reader)?.into();

        let mut pubrec = PubRec {
            packet_identifier,
            ..Default::default()
        };

        if shortened {
            pubrec.reason_code = ReasonCode::Success;
        } else {
            pubrec.reason_code =
                ReasonCode::try_parse(Byte::decode(reader)?.into(), ControlPacketType::PUBREC)?;

            let mut properties = PropertiesDecoder::take(reader)?;
            while properties.has_properties() {
                match properties.read()? {
                    Property::ReasonString(v) => pubrec.reason_string = Some(v),
                    Property::UserProperty(k, v) => pubrec.user_properties.push((k, v)),
                    _ => return Err(Error::ProtocolError),
                }
            }
        }

        Ok(pubrec)
    }
}
