use crate::{
    Byte, ControlPacketType, Decode, Encode, Error, PropertiesDecoder, Property, ReasonCode,
    Result as SageResult, TwoByteInteger, VariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Disconnect {
    pub reason_code: ReasonCode,
    pub session_expiry_interval: Option<u32>,
    pub reason_string: Option<String>,
    pub user_properties: Vec<(String, String)>,
    pub reference: Option<String>,
}

impl Default for Disconnect {
    fn default() -> Self {
        Disconnect {
            reason_code: ReasonCode::Success,
            reason_string: None,
            session_expiry_interval: None,
            user_properties: Default::default(),
            reference: None,
        }
    }
}

impl Disconnect {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = Byte(self.reason_code as u8).encode(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(v).encode(&mut properties)?;
        }
        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }if let Some(v) = self.reference {
            n_bytes += Property::ServerReference(v).encode(writer)?;
        }

        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R) -> SageResult<Self> {

        let reason_code = ReasonCode::try_parse(Byte::decode(reader)?.into(), ControlPacketType::DISCONNECT)?;
        
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(reader)?;
        let mut session_expiry_interval = None;
        let mut reason_string = None;
        let mut reference = None;

        while properties.has_properties() {
            match properties.read()? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = Some(v),
                Property::ReasonString(v) => reason_string = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                Property::ServerReference(v) => reference = Some(v),
                _ => return Err(Error::ProtocolError),
            }
        }

        Ok(Disconnect {
            reason_code,
            session_expiry_interval,
            reason_string,
            user_properties,
            reference,
        })
    }
}
