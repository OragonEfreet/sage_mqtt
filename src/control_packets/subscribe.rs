use crate::{
    Byte, Decode, Encode, Error, PropertiesDecoder, Property, QoS, Result as SageResult,
    TwoByteInteger, VariableByteInteger, DEFAULT_MAXIMUM_QOS,
};
use std::io::{Read, Write};

#[derive(Eq, Debug, PartialEq, Clone, Copy)]
pub enum RetainHandling {
    OnSubscribe = 0x00,
    OnFirstSubscribe = 0x01,
    DontSend = 0x02,
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

impl Encode for SubscriptionOptions {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        Byte(
            self.qos as u8
                | (self.no_local as u8) << 2
                | (self.retain_as_published as u8) << 3
                | (self.retain_handling as u8) << 4,
        )
        .encode(writer)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Subscribe {
    pub packet_identifier: u16,
    pub subscription_identifier: Option<u32>,
    pub user_properties: Vec<(String, String)>,
}

impl Default for Subscribe {
    fn default() -> Self {
        Subscribe {
            packet_identifier: 0,
            subscription_identifier: None,
            user_properties: Default::default(),
        }
    }
}

impl Subscribe {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = TwoByteInteger(self.packet_identifier).encode(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.subscription_identifier {
            n_bytes += VariableByteInteger(v).encode(writer)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;
        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R) -> SageResult<Self> {
        let packet_identifier = TwoByteInteger::decode(reader)?.into();

        let mut user_properties = Vec::new();
        let mut subscription_identifier = None;

        let mut properties = PropertiesDecoder::take(reader)?;
        while properties.has_properties() {
            match properties.read()? {
                Property::SubscriptionIdentifier(v) => subscription_identifier = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            }
        }

        Ok(Subscribe {
            packet_identifier,
            subscription_identifier,
            user_properties,
        })
    }
}
