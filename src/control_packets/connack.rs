use crate::{
    Authentication, ControlPacketType, Encode, Error, PropertiesDecoder, Property, QoS, ReadByte,
    ReasonCode, Result as SageResult, WriteByte, WriteVariableByteInteger, DEFAULT_MAXIMUM_QOS,
    DEFAULT_RECEIVE_MAXIMUM, DEFAULT_RETAIN_AVAILABLE, DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
};
use std::io::{Read, Write};

#[derive(PartialEq, Debug, Clone)]
pub struct ConnAck {
    pub session_present: bool,
    pub reason_code: ReasonCode,
    pub session_expiry_interval: Option<u32>,
    pub receive_maximum: u16,
    pub maximum_qos: QoS,
    pub retain_available: bool,
    pub maximum_packet_size: Option<u32>,
    pub assigned_client_id: Option<String>,
    pub topic_alias_maximum: u16,
    pub reason_string: String,
    pub user_properties: Vec<(String, String)>,
    pub wildcard_subscription_available: bool,
    pub shared_subscription_available: bool,
    pub keep_alive: Option<u16>,
    pub response_information: String,
    pub reference: Option<String>,
    pub authentication: Option<Authentication>,
}

impl Default for ConnAck {
    fn default() -> Self {
        ConnAck {
            session_present: false,
            reason_code: ReasonCode::Success,
            session_expiry_interval: None,
            receive_maximum: DEFAULT_RECEIVE_MAXIMUM,
            maximum_qos: DEFAULT_MAXIMUM_QOS,
            retain_available: DEFAULT_RETAIN_AVAILABLE,
            maximum_packet_size: None,
            assigned_client_id: None,
            topic_alias_maximum: DEFAULT_TOPIC_ALIAS_MAXIMUM,
            reason_string: Default::default(),
            user_properties: Default::default(),
            wildcard_subscription_available: DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
            shared_subscription_available: DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
            keep_alive: None,
            response_information: Default::default(),
            reference: None,
            authentication: None,
        }
    }
}

impl ConnAck {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = self.session_present.write_byte(writer)?;
        n_bytes += self.reason_code.write_byte(writer)?;

        let mut properties = Vec::new();

        if let Some(v) = self.session_expiry_interval {
            n_bytes += Property::SessionExpiryInterval(v).encode(&mut properties)?;
        }
        n_bytes += Property::ReceiveMaximum(self.receive_maximum).encode(&mut properties)?;
        n_bytes += Property::MaximumQoS(self.maximum_qos).encode(&mut properties)?;
        n_bytes += Property::RetainAvailable(self.retain_available).encode(&mut properties)?;
        if let Some(v) = self.maximum_packet_size {
            n_bytes += Property::MaximumPacketSize(v).encode(&mut properties)?;
        }
        if let Some(v) = self.assigned_client_id {
            n_bytes += Property::AssignedClientIdentifier(v).encode(&mut properties)?;
        }
        n_bytes += Property::TopicAliasMaximum(self.topic_alias_maximum).encode(&mut properties)?;
        n_bytes += Property::ReasonString(self.reason_string).encode(&mut properties)?;
        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }
        n_bytes += Property::WildcardSubscriptionAvailable(self.wildcard_subscription_available)
            .encode(&mut properties)?;
        n_bytes += Property::SharedSubscriptionAvailable(self.shared_subscription_available)
            .encode(&mut properties)?;
        if let Some(v) = self.keep_alive {
            n_bytes += Property::ServerKeepAlive(v).encode(writer)?;
        }
        n_bytes += Property::ResponseInformation(self.response_information).encode(writer)?;
        if let Some(v) = self.reference {
            n_bytes += Property::ServerReference(v).encode(writer)?;
        }
        if let Some(authentication) = self.authentication {
            n_bytes +=
                Property::AuthenticationMethod(authentication.method).encode(&mut properties)?;
            if !authentication.data.is_empty() {
                n_bytes +=
                    Property::AuthenticationData(authentication.data).encode(&mut properties)?;
            }
        }

        n_bytes += properties.len().write_variable_byte_integer(writer)?;
        writer.write_all(&properties)?;

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R) -> SageResult<Self> {
        let session_present = bool::read_byte(reader)?;

        let reason_code =
            ReasonCode::try_parse(u8::read_byte(reader)?, ControlPacketType::CONNECT)?;

        let mut session_expiry_interval = None;
        let mut receive_maximum = DEFAULT_RECEIVE_MAXIMUM;
        let mut maximum_qos = DEFAULT_MAXIMUM_QOS;
        let mut retain_available = DEFAULT_RETAIN_AVAILABLE;
        let mut maximum_packet_size = None;
        let mut assigned_client_id = None;
        let mut topic_alias_maximum = DEFAULT_TOPIC_ALIAS_MAXIMUM;
        let mut reason_string = Default::default();
        let mut user_properties = Vec::new();
        let mut wildcard_subscription_available = DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE;
        let mut shared_subscription_available = DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE;
        let mut keep_alive = None;
        let mut response_information = Default::default();
        let mut reference = None;
        let mut authentication_method = None;
        let mut authentication_data = Default::default();

        let mut decoder = PropertiesDecoder::take(reader)?;
        while decoder.has_properties() {
            match decoder.read()? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = Some(v),
                Property::ReceiveMaximum(v) => receive_maximum = v,
                Property::MaximumQoS(v) => maximum_qos = v,
                Property::RetainAvailable(v) => retain_available = v,
                Property::MaximumPacketSize(v) => maximum_packet_size = Some(v),
                Property::AssignedClientIdentifier(v) => assigned_client_id = Some(v),
                Property::TopicAliasMaximum(v) => topic_alias_maximum = v,
                Property::ReasonString(v) => reason_string = v,
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                Property::WildcardSubscriptionAvailable(v) => wildcard_subscription_available = v,
                Property::SharedSubscriptionAvailable(v) => shared_subscription_available = v,
                Property::ServerKeepAlive(v) => keep_alive = Some(v),
                Property::ResponseInformation(v) => response_information = v,
                Property::ServerReference(v) => reference = Some(v),
                Property::AuthenticationMethod(v) => authentication_method = Some(v),
                Property::AuthenticationData(v) => authentication_data = v,
                _ => return Err(Error::ProtocolError),
            }
        }

        let authentication = if let Some(method) = authentication_method {
            Some(Authentication {
                method,
                data: authentication_data,
            })
        } else {
            if !authentication_data.is_empty() {
                return Err(Error::ProtocolError);
            }
            None
        };

        Ok(ConnAck {
            session_present,
            reason_code,
            session_expiry_interval,
            receive_maximum,
            maximum_qos,
            retain_available,
            maximum_packet_size,
            assigned_client_id,
            topic_alias_maximum,
            reason_string,
            user_properties,
            wildcard_subscription_available,
            shared_subscription_available,
            keep_alive,
            response_information,
            reference,
            authentication,
        })
    }
}
