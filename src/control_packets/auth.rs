use crate::{
    codec, Authentication, ControlPacketType, Error, PropertiesDecoder, Property, ReasonCode,
    Result as SageResult,
};
use std::io::{Read, Write};

/// The `Auth` packet is used for enhanced authentication upon connection.
/// When a client connects to a server, it can initiates an authentication using
/// the `Authentication` structure. Then the client and server exchange `Auth`
/// packets until either the the client sends a `Disconnect` packet or the
/// server respond with a `Connack` packet.
#[derive(Debug, PartialEq, Clone)]
pub struct Auth {
    /// The packet reason code. Can be any of:
    /// - Success: The authentication is successful
    /// - ReAuthenticate (client only): Ask for a new authentication
    /// - ContinueAuthentication: Continue the authentication with another step
    pub reason_code: ReasonCode,

    /// The `Authentication` data which consists in an authentication method and
    /// optionnaly data.
    pub authentication: Authentication,

    /// Optional reason string sent by the server.
    pub reason_string: Option<String>,

    /// General purpose user properties.
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
    pub(crate) fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_reason_code(self.reason_code, writer)?;
        let mut properties = Vec::new();

        n_bytes += self.authentication.write(&mut properties)?;
        if let Some(v) = self.reason_string {
            n_bytes += Property::ReasonString(v).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer)?;
        writer.write_all(&properties)?;

        Ok(n_bytes)
    }

    pub(crate) fn read<R: Read>(reader: &mut R) -> SageResult<Self> {
        let reason_code =
            ReasonCode::try_parse(codec::read_byte(reader)?, ControlPacketType::AUTH)?;

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

#[cfg(test)]
mod unit {

    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            24, 38, 21, 0, 6, 87, 105, 108, 108, 111, 119, 22, 0, 4, 13, 21, 234, 94, 31, 0, 4, 66,
            105, 119, 105, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116,
        ]
    }

    fn decoded() -> Auth {
        Auth {
            reason_code: ReasonCode::ContinueAuthentication,
            authentication: Authentication {
                method: "Willow".into(),
                data: vec![0x0D, 0x15, 0xEA, 0x5E],
            },
            reason_string: Some("Biwi".into()),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 40);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Auth::read(&mut test_data).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
