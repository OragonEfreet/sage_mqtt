use crate::{
    Error, PropertiesDecoder, Property, ReadTwoByteInteger, ReadUTF8String, Result as SageResult,
    WriteTwoByteInteger, WriteUTF8String, WriteVariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct UnSubscribe {
    pub packet_identifier: u16,
    pub user_properties: Vec<(String, String)>,
    pub subscriptions: Vec<String>,
}

impl Default for UnSubscribe {
    fn default() -> Self {
        UnSubscribe {
            packet_identifier: 0,
            user_properties: Default::default(),
            subscriptions: Default::default(),
        }
    }
}

impl UnSubscribe {
    pub(crate) fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_identifier.write_two_byte_integer(writer)?;

        let mut properties = Vec::new();
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }
        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        for option in self.subscriptions {
            n_bytes += option.write_utf8_string(writer)?;
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

        let mut subscriptions = Vec::new();

        while reader.limit() > 0 {
            subscriptions.push(String::read_utf8_string(&mut reader)?);
        }

        if subscriptions.is_empty() {
            Err(Error::ProtocolError)
        } else {
            Ok(UnSubscribe {
                packet_identifier,
                user_properties,
                subscriptions,
            })
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 15, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 0, 6, 104, 97,
            114, 100, 101, 114, 0, 6, 98, 101, 116, 116, 101, 114, 0, 6, 102, 97, 115, 116, 101,
            114, 0, 8, 115, 116, 114, 111, 110, 103, 101, 114,
        ]
    }

    fn decoded() -> UnSubscribe {
        UnSubscribe {
            packet_identifier: 1337,
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            subscriptions: vec![
                "harder".into(),
                "better".into(),
                "faster".into(),
                "stronger".into(),
            ],
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 52);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = UnSubscribe::read(&mut test_data, 52).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
