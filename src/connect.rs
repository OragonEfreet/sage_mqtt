use crate::{
    BinaryData, Bits, Byte, Decode, Encode, Error, PropertiesDecoder, Property, QoS,
    Result as SageResult, TwoByteInteger, UTF8String, VariableByteInteger,
};
use std::convert::TryInto;
use std::io::{Read, Write};

const DEFAULT_SESSION_EXPIRY_INTERVAL: u32 = 0;
const DEFAULT_RECEIVE_MAXIMUM: u16 = 65_535;
const DEFAULT_TOPIC_ALIAS_MAXIMUM: u16 = 0;
const DEFAULT_REQUEST_RESPONSE_INFORMATION: bool = false;
const DEFAULT_REQUEST_PROBLEM_INFORMATION: bool = true;
const DEFAULT_WILL_DELAY_INTERVAL: u32 = 0;
const DEFAULT_PAYLOAD_FORMAT_INDICATOR: bool = false;

#[derive(Debug, PartialEq, Clone)]
pub struct Will {
    // Sorted
    pub qos: QoS,
    pub retain: bool,
    pub delay_interval: u32,
    pub format_indicator: bool,
    pub message_expiry_interval: Option<u32>,
    pub content_type: String,
    pub response_topic: String,
    pub correlation_data: Option<Vec<u8>>,
    pub user_properties: Vec<(String, String)>,
    pub topic: String,
    pub payload: Vec<u8>,
}

impl Default for Will {
    fn default() -> Self {
        Will {
            qos: QoS::AtMostOnce,
            retain: false,
            delay_interval: DEFAULT_WILL_DELAY_INTERVAL,
            format_indicator: DEFAULT_PAYLOAD_FORMAT_INDICATOR,
            message_expiry_interval: None,
            content_type: Default::default(),
            response_topic: Default::default(),
            correlation_data: None,
            user_properties: Default::default(),
            topic: Default::default(),
            payload: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Authentication {
    pub method: String,
    pub data: Vec<u8>,
}

/// The `Connect` control packet is used to open a connection
#[derive(PartialEq, Debug, Clone)]
pub struct Connect {
    // Sorted
    pub clean_start: bool,
    pub user_name: Option<String>,
    pub password: Option<Vec<u8>>,
    pub keep_alive: u16,
    pub session_expiry_interval: u32,
    pub receive_maximum: u16,
    pub maximum_packet_size: Option<u32>,
    pub topic_alias_maximum: u16,
    pub request_response_information: bool,
    pub request_problem_information: bool,
    pub user_properties: Vec<(String, String)>,
    pub authentication: Option<Authentication>,
    pub client_id: String,
    pub will: Option<Will>,
}

impl Default for Connect {
    fn default() -> Self {
        Connect {
            clean_start: false,
            user_name: None,
            password: Default::default(),
            keep_alive: 300,
            session_expiry_interval: DEFAULT_SESSION_EXPIRY_INTERVAL,
            receive_maximum: DEFAULT_RECEIVE_MAXIMUM,
            maximum_packet_size: None,
            topic_alias_maximum: DEFAULT_TOPIC_ALIAS_MAXIMUM,
            request_response_information: DEFAULT_REQUEST_RESPONSE_INFORMATION,
            request_problem_information: DEFAULT_REQUEST_PROBLEM_INFORMATION,
            user_properties: Default::default(),
            authentication: None,
            client_id: Default::default(),
            will: None,
        }
    }
}

struct ConnectFlags {
    pub clean_start: bool,
    pub will: bool,
    pub will_qos: QoS,
    pub will_retain: bool,
    pub user_name: bool,
    pub password: bool,
}

impl Encode for Connect {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        // Variable Header (into content)
        let mut n_bytes = UTF8String::from("MQTT").encode(writer)?;
        n_bytes += Byte(0x05).encode(writer)?;

        n_bytes += ConnectFlags {
            clean_start: self.clean_start,
            will: self.will.is_some(),
            will_qos: if let Some(w) = &self.will {
                w.qos
            } else {
                QoS::AtMostOnce
            },
            will_retain: if let Some(w) = &self.will {
                w.retain
            } else {
                false
            },
            user_name: self.user_name.is_some(),
            password: self.password.is_some(),
        }
        .encode(writer)?;

        n_bytes += TwoByteInteger(self.keep_alive).encode(writer)?;

        // Properties
        let mut properties = Vec::new();
        if self.session_expiry_interval != DEFAULT_SESSION_EXPIRY_INTERVAL {
            n_bytes += Property::SessionExpiryInterval(self.session_expiry_interval)
                .encode(&mut properties)?;
        }
        if self.receive_maximum != DEFAULT_RECEIVE_MAXIMUM {
            n_bytes += Property::ReceiveMaximum(self.receive_maximum).encode(&mut properties)?;
        }
        if let Some(maximum_packet_size) = self.maximum_packet_size {
            n_bytes += Property::MaximumPacketSize(maximum_packet_size).encode(&mut properties)?;
        }
        if self.topic_alias_maximum != DEFAULT_TOPIC_ALIAS_MAXIMUM {
            n_bytes +=
                Property::TopicAliasMaximum(self.topic_alias_maximum).encode(&mut properties)?;
        }
        if self.request_response_information != DEFAULT_REQUEST_RESPONSE_INFORMATION {
            n_bytes += Property::RequestResponseInformation(self.request_response_information)
                .encode(&mut properties)?;
        }
        if self.request_problem_information != DEFAULT_REQUEST_PROBLEM_INFORMATION {
            n_bytes += Property::RequestResponseInformation(self.request_problem_information)
                .encode(&mut properties)?;
        }
        for property in self.user_properties {
            n_bytes += Property::UserProperty(property.0, property.1).encode(&mut properties)?;
        }
        if let Some(authentication) = self.authentication {
            n_bytes +=
                Property::AuthenticationMethod(authentication.method).encode(&mut properties)?;
            if !authentication.data.is_empty() {
                n_bytes +=
                    Property::AuthenticationData(authentication.data).encode(&mut properties)?;
            }
        }

        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;

        // Payload
        if self.client_id.len() > 23 || self.client_id.chars().any(|c| c < '0' || c > 'z') {
            return Err(Error::MalformedPacket);
        }
        n_bytes += UTF8String(self.client_id).encode(writer)?;

        if let Some(w) = self.will {
            let mut properties = Vec::new();

            if w.delay_interval != DEFAULT_WILL_DELAY_INTERVAL {
                n_bytes += Property::WillDelayInterval(w.delay_interval).encode(&mut properties)?;
            }
            if w.format_indicator != DEFAULT_PAYLOAD_FORMAT_INDICATOR {
                n_bytes +=
                    Property::PayloadFormatIndicator(w.format_indicator).encode(&mut properties)?;
            }
            if let Some(v) = w.message_expiry_interval {
                n_bytes += Property::MessageExpiryInterval(v).encode(&mut properties)?;
            }
            n_bytes += Property::ContentType(w.content_type).encode(&mut properties)?;
            n_bytes += Property::ResponseTopic(w.response_topic).encode(&mut properties)?;
            if let Some(v) = w.correlation_data {
                n_bytes += Property::CorrelationData(v).encode(&mut properties)?;
            }
            for (k, v) in w.user_properties {
                n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
            }

            n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
            writer.write_all(&properties)?;

            n_bytes += UTF8String(w.topic).encode(writer)?;
            n_bytes += BinaryData(w.payload).encode(writer)?;
        }

        if let Some(v) = self.user_name {
            n_bytes += UTF8String(v).encode(writer)?;
        }

        if let Some(v) = self.password {
            n_bytes += BinaryData(v).encode(writer)?;
        }

        Ok(n_bytes)
    }
}

impl Decode for Connect {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let protocol_name = UTF8String::decode(reader)?;
        if protocol_name.0 != "MQTT" {
            return Err(Error::MalformedPacket);
        }

        let protocol_version = Byte::decode(reader)?;
        if protocol_version.0 != 0x05 {
            return Err(Error::MalformedPacket);
        }

        let flags = ConnectFlags::decode(reader)?;

        let clean_start = flags.clean_start;

        let keep_alive = TwoByteInteger::decode(reader)?.into();

        let mut session_expiry_interval = DEFAULT_SESSION_EXPIRY_INTERVAL;
        let mut receive_maximum = DEFAULT_RECEIVE_MAXIMUM;
        let mut maximum_packet_size = None;
        let mut topic_alias_maximum = DEFAULT_TOPIC_ALIAS_MAXIMUM;
        let mut request_response_information = DEFAULT_REQUEST_RESPONSE_INFORMATION;
        let mut request_problem_information = DEFAULT_REQUEST_PROBLEM_INFORMATION;
        let mut user_properties = Vec::new();

        let mut authentication_method = None;
        let mut authentication_data = Default::default();

        let mut decoder = PropertiesDecoder::take(reader)?;

        while decoder.has_properties() {
            match decoder.read()? {
                Property::SessionExpiryInterval(v) => session_expiry_interval = v,
                Property::ReceiveMaximum(v) => receive_maximum = v,
                Property::MaximumPacketSize(v) => maximum_packet_size = Some(v),
                Property::TopicAliasMaximum(v) => topic_alias_maximum = v,
                Property::RequestResponseInformation(v) => request_response_information = v,
                Property::RequestProblemInformation(v) => request_problem_information = v,

                Property::AuthenticationMethod(v) => authentication_method = Some(v),
                Property::AuthenticationData(v) => authentication_data = v,
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            };
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

        // Payload
        let client_id = String::from(UTF8String::decode(reader)?);
        if client_id.len() > 23 || client_id.chars().any(|c| c < '0' || c > 'z') {
            return Err(Error::MalformedPacket);
        }

        let will = if flags.will {
            let mut decoder = PropertiesDecoder::take(reader)?;
            let mut w = Will::default();
            w.qos = flags.will_qos;
            while decoder.has_properties() {
                match decoder.read()? {
                    Property::WillDelayInterval(v) => w.delay_interval = v,
                    Property::PayloadFormatIndicator(v) => w.format_indicator = v,
                    Property::MessageExpiryInterval(v) => w.message_expiry_interval = Some(v),
                    Property::ContentType(v) => w.content_type = v,
                    Property::ResponseTopic(v) => w.response_topic = v,
                    Property::CorrelationData(v) => w.correlation_data = Some(v),
                    Property::UserProperty(k, v) => w.user_properties.push((k, v)),
                    _ => return Err(Error::ProtocolError),
                }
            }
            w.topic = UTF8String::decode(reader)?.into();
            w.payload = BinaryData::decode(reader)?.into();
            Some(w)
        } else {
            None
        };

        let user_name = if flags.user_name {
            Some(UTF8String::decode(reader)?.into())
        } else {
            None
        };

        let password = if flags.password {
            Some(BinaryData::decode(reader)?.into())
        } else {
            None
        };

        Ok(Connect {
            clean_start,
            user_name,
            password,
            keep_alive,
            session_expiry_interval,
            receive_maximum,
            maximum_packet_size,
            topic_alias_maximum,
            request_response_information,
            request_problem_information,
            authentication,
            user_properties,
            client_id,
            will,
        })
    }
}

#[cfg(test)]
mod unit_connect {

    use std::io::Cursor;

    use super::*;

    // Keep Alive MSB (0)
    // Keep Alive MSB (10)
    // Properties:
    // Session Expiry Interface Identifier (17)
    // Session Expiry Interval (10)
    const CONNECT_ENCODED: [u8; 16] = [
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x05, 0xCE, 0x00, 0x0A, 0x05, 0x11, 0x00, 0x00, 0x00,
        0x0A,
    ];

    fn connect_decoded() -> Connect {
        let keep_alive = 10;
        let session_expiry_interval = 10;

        Connect {
            keep_alive,
            session_expiry_interval,
            ..Default::default()
        }
    }

    #[test]
    fn encode_control_connect() {
        let connect = connect_decoded();
        let mut encoded = Vec::new();

        let n_bytes = connect.encode(&mut encoded).unwrap();
        assert_eq!(encoded, CONNECT_ENCODED);
        assert_eq!(n_bytes, 16);
    }

    #[test]
    fn decode_control_connect() {
        let mut test_stream = Cursor::new(CONNECT_ENCODED);
        let connect = Connect::decode(&mut test_stream).unwrap();
        assert_eq!(connect, connect_decoded());
    }
}

impl Encode for ConnectFlags {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let bits = ((self.user_name as u8) << 7)
            | ((self.password as u8) << 6)
            | ((self.will_retain as u8) << 5)
            | (self.will_qos as u8) << 3
            | ((self.will as u8) << 2)
            | ((self.clean_start as u8) << 1);
        Ok(Bits(bits).encode(writer)?)
    }
}

impl Decode for ConnectFlags {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let bits: u8 = Bits::decode(reader)?.into();

        if bits & 0x01 != 0 {
            Err(Error::MalformedPacket)
        } else {
            Ok(ConnectFlags {
                user_name: (bits & 0b1000_0000) >> 7 > 0,
                password: (bits & 0b0100_0000) >> 6 > 0,
                will_retain: (bits & 0b0010_0000) >> 5 > 0,
                will_qos: ((bits & 0b0001_1000) >> 3).try_into()?,
                will: (bits & 0b0000_00100) >> 2 > 0,
                clean_start: (bits & 0b0000_00010) >> 1 > 0,
            })
        }
    }
}
