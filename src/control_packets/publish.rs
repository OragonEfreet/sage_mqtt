use crate::{
    Encode, Error, PropertiesDecoder, Property, QoS, ReadTwoByteInteger, ReadUTF8String,
    Result as SageResult, WriteTwoByteInteger, WriteUTF8String, WriteVariableByteInteger,
    DEFAULT_PAYLOAD_FORMAT_INDICATOR,
};

use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Publish {
    pub duplicate: bool,
    pub qos: QoS,
    pub retain: bool,
    pub topic_name: String,
    pub packet_identifier: Option<u16>,
    pub payload_format_indicator: bool,
    pub message_expiry_interval: Option<u32>,
    pub topic_alias: Option<u16>,
    pub response_topic: Option<String>,
    pub correlation_data: Option<Vec<u8>>,
    pub user_properties: Vec<(String, String)>,
    pub subscription_identifiers: Vec<u32>,
    pub content_type: String,
    pub application_message: Vec<u8>,
}

impl Default for Publish {
    fn default() -> Self {
        Publish {
            duplicate: false,
            qos: QoS::ExactlyOnce,
            retain: false,
            topic_name: Default::default(),
            packet_identifier: None,
            payload_format_indicator: DEFAULT_PAYLOAD_FORMAT_INDICATOR,
            message_expiry_interval: None,
            topic_alias: None,
            response_topic: None,
            correlation_data: None,
            user_properties: Default::default(),
            subscription_identifiers: Default::default(),
            content_type: Default::default(),
            application_message: Default::default(),
        }
    }
}

impl Publish {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.topic_name.write_utf8_string(writer)?;

        if self.qos != QoS::AtMostOnce {
            if let Some(packet_identifier) = self.packet_identifier {
                n_bytes += packet_identifier.write_two_byte_integer(writer)?;
            } else {
                return Err(Error::ProtocolError);
            }
        }

        let mut properties = Vec::new();
        n_bytes += Property::PayloadFormatIndicator(self.payload_format_indicator)
            .encode(&mut properties)?;
        if let Some(message_expiry_interval) = self.message_expiry_interval {
            n_bytes +=
                Property::MessageExpiryInterval(message_expiry_interval).encode(&mut properties)?;
        }
        if let Some(topic_alias) = self.topic_alias {
            n_bytes += Property::TopicAlias(topic_alias).encode(&mut properties)?;
        }
        if let Some(response_topic) = self.response_topic {
            n_bytes += Property::ResponseTopic(response_topic).encode(&mut properties)?;
        }
        if let Some(correlation_data) = self.correlation_data {
            n_bytes += Property::CorrelationData(correlation_data).encode(&mut properties)?;
        }
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }
        for v in self.subscription_identifiers {
            n_bytes += Property::SubscriptionIdentifier(v).encode(&mut properties)?;
        }
        n_bytes += Property::ContentType(self.content_type).encode(&mut properties)?;

        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        n_bytes += writer.write(&self.application_message)?;

        Ok(n_bytes)
    }

    pub fn read<R: Read>(
        reader: &mut R,
        duplicate: bool,
        qos: QoS,
        retain: bool,
        remaining_size: u64,
    ) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size);

        let topic_name = String::read_utf8_string(&mut reader)?;

        let packet_identifier = if qos != QoS::AtMostOnce {
            Some(u16::read_two_byte_integer(&mut reader)?)
        } else {
            None
        };

        let mut payload_format_indicator = DEFAULT_PAYLOAD_FORMAT_INDICATOR;
        let mut message_expiry_interval = None;
        let mut topic_alias = None;
        let mut response_topic = None;
        let mut correlation_data = None;
        let mut user_properties = Vec::new();
        let mut subscription_identifiers = Vec::new();
        let mut content_type = Default::default();

        let mut properties = PropertiesDecoder::take(&mut reader)?;
        while properties.has_properties() {
            match properties.read()? {
                Property::PayloadFormatIndicator(v) => payload_format_indicator = v,
                Property::MessageExpiryInterval(v) => message_expiry_interval = Some(v),
                Property::TopicAlias(v) => topic_alias = Some(v),
                Property::ResponseTopic(v) => response_topic = Some(v),
                Property::CorrelationData(v) => correlation_data = Some(v),
                Property::UserProperty(v, k) => user_properties.push((k, v)),
                Property::SubscriptionIdentifier(v) => subscription_identifiers.push(v),
                Property::ContentType(v) => content_type = v,
                _ => return Err(Error::ProtocolError),
            }
        }

        let mut application_message = Vec::new();
        reader.read_to_end(&mut application_message)?;

        Ok(Publish {
            duplicate,
            qos,
            retain,
            topic_name,
            packet_identifier,
            payload_format_indicator,
            message_expiry_interval,
            topic_alias,
            response_topic,
            correlation_data,
            user_properties,
            subscription_identifiers,
            content_type,
            application_message,
        })
    }
}
