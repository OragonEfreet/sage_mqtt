use crate::{
    Error, PropertiesDecoder, Property, QoS, ReadByte, ReadTwoByteInteger, ReadUTF8String,
    Result as SageResult, WriteByte, WriteTwoByteInteger, WriteUTF8String,
    WriteVariableByteInteger, DEFAULT_MAXIMUM_QOS,
};
use std::{
    convert::{TryFrom, TryInto},
    io::{Read, Write},
};

#[derive(Eq, Debug, PartialEq, Clone, Copy)]
pub enum RetainHandling {
    OnSubscribe = 0x00,
    OnFirstSubscribe = 0x01,
    DontSend = 0x02,
}

impl TryFrom<u8> for RetainHandling {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RetainHandling::OnSubscribe),
            0x01 => Ok(RetainHandling::OnFirstSubscribe),
            0x02 => Ok(RetainHandling::DontSend),
            _ => Err(Self::Error::MalformedPacket),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SubscriptionOptions {
    pub qos: QoS,
    pub no_local: bool,
    pub retain_as_published: bool,
    pub retain_handling: RetainHandling,
}

impl Default for SubscriptionOptions {
    fn default() -> Self {
        SubscriptionOptions {
            qos: DEFAULT_MAXIMUM_QOS,
            no_local: false,
            retain_as_published: false,
            retain_handling: RetainHandling::OnSubscribe,
        }
    }
}

impl SubscriptionOptions {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let byte: u8 = self.qos as u8
            | (self.no_local as u8) << 2
            | (self.retain_as_published as u8) << 3
            | (self.retain_handling as u8) << 4;
        byte.write_byte(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let flags = u8::read_byte(reader)?;
        if flags & 0b1100_0000 > 0 {
            Err(Error::ProtocolError)
        } else {
            Ok(SubscriptionOptions {
                qos: (flags & 0b0000_0011).try_into()?,
                no_local: (flags & 0b0000_0100) > 0,
                retain_as_published: (flags & 0b0000_1000) > 0,
                retain_handling: ((flags & 0b0011_0000) >> 4).try_into()?,
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Subscribe {
    pub packet_identifier: u16,
    pub subscription_identifier: Option<u32>,
    pub user_properties: Vec<(String, String)>,
    pub subscriptions: Vec<(String, SubscriptionOptions)>,
}

impl Default for Subscribe {
    fn default() -> Self {
        Subscribe {
            packet_identifier: 0,
            subscription_identifier: None,
            user_properties: Default::default(),
            subscriptions: Default::default(),
        }
    }
}

impl Subscribe {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.packet_identifier.write_two_byte_integer(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.subscription_identifier {
            n_bytes += Property::SubscriptionIdentifier(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        for option in self.subscriptions {
            n_bytes += option.0.write_utf8_string(writer)?;
            n_bytes += option.1.encode(writer)?;
        }

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R, remaining_size: usize) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);
        let packet_identifier = u16::read_two_byte_integer(&mut reader)?;

        let mut user_properties = Vec::new();
        let mut subscription_identifier = None;

        let mut properties = PropertiesDecoder::take(&mut reader)?;
        while properties.has_properties() {
            match properties.read()? {
                Property::SubscriptionIdentifier(v) => subscription_identifier = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            }
        }

        //        return Err(Error::DebugMarker("TOUT VA BIEN".into()));

        let mut subscriptions = Vec::new();

        while reader.limit() > 0 {
            subscriptions.push((
                String::read_utf8_string(&mut reader)?,
                SubscriptionOptions::decode(&mut reader)?,
            ));
        }

        if subscriptions.is_empty() {
            Err(Error::ProtocolError)
        } else {
            Ok(Subscribe {
                packet_identifier,
                subscription_identifier,
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
            5, 57, 18, 11, 195, 3, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 0,
            6, 104, 97, 114, 100, 101, 114, 1, 0, 6, 98, 101, 116, 116, 101, 114, 20, 0, 6, 102,
            97, 115, 116, 101, 114, 14, 0, 8, 115, 116, 114, 111, 110, 103, 101, 114, 41,
        ]
    }

    fn decoded() -> Subscribe {
        Subscribe {
            packet_identifier: 1337,
            subscription_identifier: Some(451),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            subscriptions: vec![
                (
                    "harder".into(),
                    SubscriptionOptions {
                        qos: QoS::AtLeastOnce,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: RetainHandling::OnSubscribe,
                    },
                ),
                (
                    "better".into(),
                    SubscriptionOptions {
                        qos: QoS::AtMostOnce,
                        no_local: true,
                        retain_as_published: false,
                        retain_handling: RetainHandling::OnFirstSubscribe,
                    },
                ),
                (
                    "faster".into(),
                    SubscriptionOptions {
                        qos: QoS::ExactlyOnce,
                        no_local: true,
                        retain_as_published: true,
                        retain_handling: RetainHandling::OnSubscribe,
                    },
                ),
                (
                    "stronger".into(),
                    SubscriptionOptions {
                        qos: QoS::AtLeastOnce,
                        no_local: false,
                        retain_as_published: true,
                        retain_handling: RetainHandling::DontSend,
                    },
                ),
            ],
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 59);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Subscribe::read(&mut test_data, 59).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
