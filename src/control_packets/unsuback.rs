use crate::{
    codec, ControlPacketType, Error, PropertiesDecoder, Property, ReasonCode, Result as SageResult,
};
use async_std::io::{
    prelude::{ReadExt, WriteExt},
    Read, Write,
};
use std::marker::Unpin;

/// An `UnSubAck` is sent by the server to acknowledge an unsubscribe request.
#[derive(Debug, PartialEq, Clone)]
pub struct UnSubAck {
    /// The packet identifier is used to identify the message throughout the
    /// communication
    pub packet_identifier: u16,

    /// An optional description of the acknowledgement.
    pub reason_string: Option<String>,

    /// General purpose user-defined properties
    pub user_properties: Vec<(String, String)>,

    /// A list of reason codes ackowledging the unsubscribtion.
    /// Each `ReasonCode` at a given index correspond to a unsubscribe request
    /// from the `Unsubscribe` packet at the same index.
    pub reason_codes: Vec<ReasonCode>,
}

impl Default for UnSubAck {
    fn default() -> Self {
        UnSubAck {
            packet_identifier: 0,
            reason_string: None,
            user_properties: Default::default(),
            reason_codes: Default::default(),
        }
    }
}

impl UnSubAck {
    /// Write the `UnSubAck` body of a packet, returning the written size in bytes
    /// in case of success.
    pub async fn write<W: Write + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, writer).await?;

        let mut properties = Vec::new();

        if let Some(reason_string) = self.reason_string {
            n_bytes += Property::ReasonString(reason_string)
                .encode(&mut properties)
                .await?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
        writer.write_all(&properties).await?;

        for reason_code in self.reason_codes {
            n_bytes += codec::write_reason_code(reason_code, writer).await?;
        }

        Ok(n_bytes)
    }

    /// Read the `UnSubAck` body from `reader`, retuning it in case of success.
    pub async fn read<R: Read + Unpin>(reader: &mut R, remaining_size: usize) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);

        let packet_identifier = codec::read_two_byte_integer(&mut reader).await?;
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(&mut reader).await?;
        let mut reason_string = None;
        while properties.has_properties() {
            match properties.read().await? {
                Property::ReasonString(v) => reason_string = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            }
        }

        let mut reason_codes = Vec::new();

        while reader.limit() > 0 {
            reason_codes.push(ReasonCode::try_parse(
                codec::read_byte(&mut reader).await?,
                ControlPacketType::SUBACK,
            )?);
        }

        Ok(UnSubAck {
            packet_identifier,
            user_properties,
            reason_string,
            reason_codes,
        })
    }
}

#[cfg(test)]
mod unit {
    use super::*;
    use async_std::io::Cursor;

    fn encoded() -> Vec<u8> {
        vec![
            5, 57, 36, 31, 0, 18, 71, 105, 111, 114, 103, 105, 111, 32, 98, 121, 32, 77, 111, 114,
            111, 100, 101, 114, 38, 0, 7, 77, 111, 103, 119, 97, 195, 175, 0, 3, 67, 97, 116, 145,
            143,
        ]
    }

    fn decoded() -> UnSubAck {
        UnSubAck {
            packet_identifier: 1337,
            reason_string: Some("Giorgio by Moroder".into()),
            user_properties: vec![("Mogwaï".into(), "Cat".into())],
            reason_codes: vec![
                ReasonCode::PacketIdentifierInUse,
                ReasonCode::TopicFilterInvalid,
            ],
        }
    }

    #[async_std::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 41);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = UnSubAck::read(&mut test_data, 41).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
