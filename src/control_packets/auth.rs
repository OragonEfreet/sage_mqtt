use crate::{
    Authentication, Byte, ControlPacketType, Decode, Encode, Error, PropertiesDecoder, Property,
    ReasonCode, Result as SageResult, VariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Auth {
    pub reason_code: ReasonCode,
    pub authentication: Authentication,
    pub reason_string: Option<String>,
    pub user_properties: Vec<(String, String)>,
}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            reason_code: ReasonCode::Success,
            authentication: Default::default(),
            reason_string: None,
            user_properties: Default::default(),
        }
    }
}

impl Auth {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = Byte(self.reason_code as u8).encode(writer)?;

        let mut properties = Vec::new();

        n_bytes += self.authentication.encode(&mut properties)?;
        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R) -> SageResult<Self> {
        let reason_code =
            ReasonCode::try_parse(Byte::decode(reader)?.into(), ControlPacketType::AUTH)?;

        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(reader)?;
        let mut reason_string = None;
        let mut authentication_method = None;
        let mut authentication_data = Default::default();

        while properties.has_properties() {
            match properties.read()? {
                Property::ReasonString(v) => reason_string = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                Property::AuthenticationMethod(v) => authentication_method = Some(v),
                Property::AuthenticationData(v) => authentication_data = v,
                _ => return Err(Error::ProtocolError),
            }
        }

        if let Some(method) = authentication_method {
            let authentication = Authentication {
                method,
                data: authentication_data,
            };

            Ok(Auth {
                reason_code,
                reason_string,
                authentication,
                user_properties,
            })
        } else {
            Err(Error::ProtocolError)
        }
    }
}
