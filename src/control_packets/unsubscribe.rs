use crate::{
    Decode, Encode, Error, PropertiesDecoder, Property, ReadTwoByteInteger, Result as SageResult,
    UTF8String, VariableByteInteger, WriteTwoByteInteger,
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
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_identifier.write_two_byte_integer(writer)?;

        let mut properties = Vec::new();
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }
        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;

        for option in self.subscriptions {
            n_bytes += UTF8String(option).encode(writer)?;
        }

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R, remaining_size: usize) -> SageResult<Self> {
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
            subscriptions.push(UTF8String::decode(&mut reader)?.into());
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
