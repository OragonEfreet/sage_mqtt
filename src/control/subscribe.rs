use crate::{
    codec,
    defaults::DEFAULT_MAXIMUM_QOS,
    Error, PropertiesDecoder, Property, QoS,
    ReasonCode::{MalformedPacket, ProtocolError},
    Result as SageResult, TopicFilter,
};
use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::{
    convert::{TryFrom, TryInto},
    marker::Unpin,
};

/// This option specifies whether retained messages are sent when the
/// subscription is established;
#[derive(Eq, Debug, PartialEq, Clone, Copy)]
pub enum RetainHandling {
    /// Send retained messages at the time of the subscribe
    OnSubscribe = 0x00,

    /// Send retained messages at the time of the first subscribe
    OnFirstSubscribe = 0x01,

    /// Don't send retained messages
    DontSend = 0x02,
}

impl TryFrom<u8> for RetainHandling {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(RetainHandling::OnSubscribe),
            0x01 => Ok(RetainHandling::OnFirstSubscribe),
            0x02 => Ok(RetainHandling::DontSend),
            _ => Err(MalformedPacket.into()),
        }
    }
}

/// Options used to describe a specific subscription.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SubscriptionOptions {
    /// The maximum quality of service the client is expected to receive
    /// messages.
    pub qos: QoS,

    /// If the value is `true`, messages must not be forwarded to a connection
    /// with a ClientID equal to the client id of the publishing connection.
    pub no_local: bool,

    /// If `true` messages forwarded using this subscription will keep their
    /// retain flag unchanged. Otherwise they won't be retain messages anymore.
    pub retain_as_published: bool,

    /// How retain messages are handled upon subscription.
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
    async fn encode<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let byte: u8 = self.qos as u8
            | (self.no_local as u8) << 2
            | (self.retain_as_published as u8) << 3
            | (self.retain_handling as u8) << 4;
        codec::write_byte(byte, writer).await
    }

    async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<Self> {
        let flags = codec::read_byte(reader).await?;
        if flags & 0b1100_0000 > 0 {
            Err(ProtocolError.into())
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

/// The subscribe packet is a request from the client to listen to one or more
/// topics.
#[derive(Debug, PartialEq, Clone)]
pub struct Subscribe {
    /// The packet identifier is used to identify the message throughout the
    /// communication.
    pub packet_identifier: u16,

    /// Optional identifier used to represent the subscription in nextcoming
    /// mmessages.
    pub subscription_identifier: Option<u32>,

    /// General purpose user properies
    pub user_properties: Vec<(String, String)>,

    /// The list of topics to subscribe to with options.
    /// Each topics can use wildcards.
    pub subscriptions: Vec<(TopicFilter, SubscriptionOptions)>,
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
    pub(crate) async fn write<W: AsyncWrite + Unpin>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = codec::write_two_byte_integer(self.packet_identifier, writer).await?;

        let mut properties = Vec::new();

        if let Some(v) = self.subscription_identifier {
            n_bytes += Property::SubscriptionIdentifier(v)
                .encode(&mut properties)
                .await?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties).await?;
        }

        n_bytes += codec::write_variable_byte_integer(properties.len() as u32, writer).await?;
        writer.write_all(&properties).await?;

        for option in self.subscriptions {
            n_bytes += codec::write_utf8_string(option.0.as_ref(), writer).await?;
            n_bytes += option.1.encode(writer).await?;
        }

        Ok(n_bytes)
    }

    pub(crate) async fn read<R: AsyncRead + Unpin>(
        reader: &mut R,
        remaining_size: usize,
    ) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);
        let packet_identifier = codec::read_two_byte_integer(&mut reader).await?;

        let mut user_properties = Vec::new();
        let mut subscription_identifier = None;

        let mut properties = PropertiesDecoder::take(&mut reader).await?;
        while properties.has_properties() {
            match properties.read().await? {
                Property::SubscriptionIdentifier(v) => subscription_identifier = Some(v),
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(ProtocolError.into()),
            }
        }

        let mut subscriptions = Vec::new();

        while reader.limit() > 0 {
            subscriptions.push((
                codec::read_utf8_string(&mut reader).await?.try_into()?,
                SubscriptionOptions::decode(&mut reader).await?,
            ));
        }

        if subscriptions.is_empty() {
            Err(ProtocolError.into())
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
    use async_std::io::Cursor;

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
                    "harder".try_into().unwrap(),
                    SubscriptionOptions {
                        qos: QoS::AtLeastOnce,
                        no_local: false,
                        retain_as_published: false,
                        retain_handling: RetainHandling::OnSubscribe,
                    },
                ),
                (
                    "better".try_into().unwrap(),
                    SubscriptionOptions {
                        qos: QoS::AtMostOnce,
                        no_local: true,
                        retain_as_published: false,
                        retain_handling: RetainHandling::OnFirstSubscribe,
                    },
                ),
                (
                    "faster".try_into().unwrap(),
                    SubscriptionOptions {
                        qos: QoS::ExactlyOnce,
                        no_local: true,
                        retain_as_published: true,
                        retain_handling: RetainHandling::OnSubscribe,
                    },
                ),
                (
                    "stronger".try_into().unwrap(),
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

    #[async_std::test]
    async fn encode() {
        let test_data = decoded();
        let mut tested_result = Vec::new();
        let n_bytes = test_data.write(&mut tested_result).await.unwrap();
        assert_eq!(tested_result, encoded());
        assert_eq!(n_bytes, 59);
    }

    #[async_std::test]
    async fn decode() {
        let mut test_data = Cursor::new(encoded());
        let tested_result = Subscribe::read(&mut test_data, 59).await.unwrap();
        assert_eq!(tested_result, decoded());
    }
}
