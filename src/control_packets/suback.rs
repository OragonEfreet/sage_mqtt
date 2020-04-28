use crate::{
    ControlPacketType, Error, PropertiesDecoder, Property, ReadByte, ReadTwoByteInteger,
    ReasonCode, Result as SageResult, WriteByte, WriteTwoByteInteger, WriteVariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct SubAck {
    pub packet_identifier: u16,
    pub user_properties: Vec<(String, String)>,
    pub reason_codes: Vec<ReasonCode>,
}

impl Default for SubAck {
    fn default() -> Self {
        SubAck {
            packet_identifier: 0,
            user_properties: Default::default(),
            reason_codes: Default::default(),
        }
    }
}

impl SubAck {
    pub(crate) fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_identifier.write_two_byte_integer(writer)?;

        let mut properties = Vec::new();

        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        for reason_code in self.reason_codes {
            n_bytes += reason_code.write_byte(writer)?;
        }

        Ok(n_bytes)
    }

    pub(crate) fn read<R: Read>(reader: &mut R, remaining_size: usize) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);

        let packet_identifier = u16::read_two_byte_integer(&mut reader)?;
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(&mut reader)?;
        while properties.has_properties() {
            match properties.read()? {
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            }
        }

        let mut reason_codes = Vec::new();

        while reader.limit() > 0 {
            reason_codes.push(ReasonCode::try_parse(
                u8::read_byte(&mut reader)?,
                ControlPacketType::SUBACK,
            )?);
        }

        Ok(SubAck {
            packet_identifier,
            user_properties,
            reason_codes,
        })
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 15, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 145, 143,
        ]
    }

    fn decoded() -> SubAck {
        SubAck {
            packet_identifier: 1337,
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            reason_codes: vec![
                ReasonCode::PacketIdentifierInUse,
                ReasonCode::TopicFilterInvalid,
            ],
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 20);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = SubAck::read(&mut test_data, 20).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
