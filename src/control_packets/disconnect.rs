use crate::{
    ControlPacketType, Error, PropertiesDecoder, Property, ReadByte, ReasonCode,
    Result as SageResult, WriteByte, WriteVariableByteInteger,
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
        let mut n_bytes = self.reason_code.write_byte(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(v).encode(&mut properties)?;
        }
        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }
        if let Some(v) = self.reference {
            n_bytes += Property::ServerReference(v).encode(&mut properties)?;
        }

        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R) -> SageResult<Self> {
        let reason_code =
            ReasonCode::try_parse(u8::read_byte(reader)?, ControlPacketType::DISCONNECT)?;
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

#[cfg(test)]
mod unit {

    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            150, 74, 17, 0, 0, 5, 57, 31, 0, 22, 76, 111, 115, 101, 32, 89, 111, 117, 114, 115,
            101, 108, 102, 32, 116, 111, 32, 68, 97, 110, 99, 101, 38, 0, 4, 68, 97, 102, 116, 0,
            4, 80, 117, 110, 107, 38, 0, 8, 80, 104, 97, 114, 114, 101, 108, 108, 0, 8, 87, 105,
            108, 108, 105, 97, 109, 115, 28, 0, 7, 67, 111, 109, 101, 32, 111, 110,
        ]
    }

    fn decoded() -> Disconnect {
        Disconnect {
            reason_code: ReasonCode::MessageRateTooHigh,
            session_expiry_interval: Some(1337),
            reason_string: Some("Lose Yourself to Dance".into()),
            user_properties: vec![
                ("Daft".into(), "Punk".into()),
                ("Pharrell".into(), "Williams".into()),
            ],
            reference: Some("Come on".into()),
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 76);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Disconnect::read(&mut test_data).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
