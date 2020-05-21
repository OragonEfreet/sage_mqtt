use crate::{
    codec, ControlPacketType, Error, PropertiesDecoder, Property, ReasonCode, Result as SageResult,
};
use std::io::{Read, Write};

/// A `PubAck` is the response for a `Publish` message with `AtLeastOnce` as
/// quality of service.
#[derive(Debug, PartialEq, Clone)]
pub struct PubAck {
    /// The packet identifier is used to identify the message throughout the
    /// communication.
    pub packet_identifier: u16,

    /// The reason code for the acknowledgement. Can be any of:
    /// - `Success`
    /// - `NoMatchingSubscribers`
    /// - `UnspecifiedError`
    /// - `ImplementationSpecificError`
    /// - `NotAuthorized`
    /// - `TopicNameInvalid`
    /// - `PacketIdentifierInUse`
    /// - `QuotaExceeded`
    /// - `PayloadFormatInvalid`
    pub reason_code: ReasonCode,

    /// If available, the reason string describing the acknowledgement.
    pub reason_string: Option<String>,

    /// General purpose user properties
    pub user_properties: Vec<(String, String)>,
}

impl Default for PubAck {
    fn default() -> Self {
        PubAck {
            packet_identifier: 0,
            reason_code: ReasonCode::Success,
            reason_string: None,
            user_properties: Default::default(),
        }
    }
}

impl PubAck {
    pub(crate) fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, writer)?;

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
            n_bytes += codec::write_reason_code(self.reason_code, writer)?;
            n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer)?;
            writer.write_all(&properties)?;
            Ok(n_bytes)
        }
    }

    pub(crate) fn read<R: Read>(reader: &mut R, shortened: bool) -> SageResult<Self> {
        let packet_identifier = codec::read_two_byte_integer(reader)?;

        let mut puback = PubAck {
            packet_identifier,
            ..Default::default()
        };

        if shortened {
            puback.reason_code = ReasonCode::Success;
        } else {
            puback.reason_code =
                ReasonCode::try_parse(codec::read_byte(reader)?, ControlPacketType::PUBACK)?;

            let mut properties = PropertiesDecoder::take(reader)?;
            while properties.has_properties() {
                match properties.read()? {
                    Property::ReasonString(v) => puback.reason_string = Some(v),
                    Property::UserProperty(k, v) => puback.user_properties.push((k, v)),
                    _ => return Err(Error::ProtocolError),
                }
            }
        }

        Ok(puback)
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 151, 29, 31, 0, 11, 66, 108, 97, 99, 107, 32, 66, 101, 116, 116, 121, 38, 0, 7,
            77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116,
        ]
    }

    fn decoded() -> PubAck {
        PubAck {
            packet_identifier: 1337,
            reason_code: ReasonCode::QuotaExceeded,
            reason_string: Some("Black Betty".into()),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
        }
    }

    #[test]
    fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 33);
    }

    #[test]
    fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = PubAck::read(&mut test_data, false).unwrap();
        assert_eq!(tested_result, decoded());
    }
}
