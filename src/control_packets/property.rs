use crate::{
    codec, Error, QoS, Result as SageResult, DEFAULT_MAXIMUM_QOS, DEFAULT_PAYLOAD_FORMAT_INDICATOR,
    DEFAULT_RECEIVE_MAXIMUM, DEFAULT_REQUEST_PROBLEM_INFORMATION,
    DEFAULT_REQUEST_RESPONSE_INFORMATION, DEFAULT_RETAIN_AVAILABLE,
    DEFAULT_SESSION_EXPIRY_INTERVAL, DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
    DEFAULT_WILL_DELAY_INTERVAL,
};
use std::{
    collections::HashSet,
    io::{Read, Take, Write},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum PropertyId {
    PayloadFormatIndicator = 0x01,
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0B,
    SessionExpiryInterval = 0x11,
    AssignedClientIdentifier = 0x12,
    ServerKeepAlive = 0x13,
    AuthenticationMethod = 0x15,
    AuthenticationData = 0x16,
    RequestProblemInformation = 0x17,
    WillDelayInterval = 0x18,
    RequestResponseInformation = 0x19,
    ResponseInformation = 0x1A,
    ServerReference = 0x1C,
    ReasonString = 0x1F,
    ReceiveMaximum = 0x21,
    TopicAliasMaximum = 0x22,
    TopicAlias = 0x23,
    MaximumQoS = 0x24,
    RetainAvailable = 0x25,
    UserProperty = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifiersAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2A,
}

fn write_property_id<W: Write>(id: PropertyId, writer: &mut W) -> SageResult<usize> {
    codec::write_variable_byte_integer(id as u32, writer)
}

fn read_property_id<R: Read>(reader: &mut R) -> SageResult<PropertyId> {
    match codec::read_variable_byte_integer(reader)? {
        0x01 => Ok(PropertyId::PayloadFormatIndicator),
        0x02 => Ok(PropertyId::MessageExpiryInterval),
        0x03 => Ok(PropertyId::ContentType),
        0x08 => Ok(PropertyId::ResponseTopic),
        0x09 => Ok(PropertyId::CorrelationData),
        0x0B => Ok(PropertyId::SubscriptionIdentifier),
        0x11 => Ok(PropertyId::SessionExpiryInterval),
        0x12 => Ok(PropertyId::AssignedClientIdentifier),
        0x13 => Ok(PropertyId::ServerKeepAlive),
        0x15 => Ok(PropertyId::AuthenticationMethod),
        0x16 => Ok(PropertyId::AuthenticationData),
        0x17 => Ok(PropertyId::RequestProblemInformation),
        0x18 => Ok(PropertyId::WillDelayInterval),
        0x19 => Ok(PropertyId::RequestResponseInformation),
        0x1A => Ok(PropertyId::ResponseInformation),
        0x1C => Ok(PropertyId::ServerReference),
        0x1F => Ok(PropertyId::ReasonString),
        0x21 => Ok(PropertyId::ReceiveMaximum),
        0x22 => Ok(PropertyId::TopicAliasMaximum),
        0x23 => Ok(PropertyId::TopicAlias),
        0x24 => Ok(PropertyId::MaximumQoS),
        0x25 => Ok(PropertyId::RetainAvailable),
        0x26 => Ok(PropertyId::UserProperty),
        0x27 => Ok(PropertyId::MaximumPacketSize),
        0x28 => Ok(PropertyId::WildcardSubscriptionAvailable),
        0x29 => Ok(PropertyId::SubscriptionIdentifiersAvailable),
        0x2A => Ok(PropertyId::SharedSubscriptionAvailable),
        _ => Err(Error::ProtocolError),
    }
}

#[derive(Debug, PartialEq)]
pub enum Property {
    PayloadFormatIndicator(bool),
    MessageExpiryInterval(u32),
    ContentType(String),
    ResponseTopic(String),
    CorrelationData(Vec<u8>),
    SubscriptionIdentifier(u32),
    SessionExpiryInterval(u32),
    AssignedClientIdentifier(String),
    ServerKeepAlive(u16),
    AuthenticationMethod(String),
    AuthenticationData(Vec<u8>),
    RequestProblemInformation(bool),
    WillDelayInterval(u32),
    RequestResponseInformation(bool),
    ResponseInformation(String),
    ServerReference(String),
    ReasonString(String),
    ReceiveMaximum(u16),
    TopicAliasMaximum(u16),
    TopicAlias(u16),
    MaximumQoS(QoS),
    RetainAvailable(bool),
    UserProperty(String, String),
    MaximumPacketSize(u32),
    WildcardSubscriptionAvailable(bool),
    SubscriptionIdentifiersAvailable(bool),
    SharedSubscriptionAvailable(bool),
}

pub struct PropertiesDecoder<'a, R: Read> {
    reader: Take<&'a mut R>,
    marked: HashSet<PropertyId>,
}

impl<'a, R: Read> PropertiesDecoder<'a, R> {
    pub fn take(reader: &'a mut R) -> SageResult<Self> {
        let len = codec::read_variable_byte_integer(reader)? as u64;
        Ok(PropertiesDecoder {
            reader: reader.take(len),
            marked: HashSet::new(),
        })
    }

    pub fn has_properties(&self) -> bool {
        self.reader.limit() > 0
    }

    pub fn read(&mut self) -> SageResult<Property> {
        let reader = &mut self.reader;
        let property_id = read_property_id(reader)?;

        // Filter by authorized properties and unicity requirements
        if (property_id != PropertyId::UserProperty
            && property_id != PropertyId::SubscriptionIdentifier)
            && !self.marked.insert(property_id)
        {
            return Err(Error::ProtocolError);
        }
        self.read_property_value(property_id)
    }

    fn read_property_value(&mut self, id: PropertyId) -> SageResult<Property> {
        let reader = &mut self.reader;
        match id {
            PropertyId::PayloadFormatIndicator => match codec::read_byte(reader)? {
                0x00 => Ok(Property::PayloadFormatIndicator(false)),
                0x01 => Ok(Property::PayloadFormatIndicator(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::MessageExpiryInterval => Ok(Property::MessageExpiryInterval(
                codec::read_four_byte_integer(reader)?,
            )),
            PropertyId::ContentType => Ok(Property::ContentType(codec::read_utf8_string(reader)?)),
            PropertyId::ResponseTopic => {
                let topic = codec::read_utf8_string(reader)?;
                if topic.is_empty() {
                    Err(Error::ProtocolError)
                } else {
                    Ok(Property::ResponseTopic(topic))
                }
            }
            PropertyId::CorrelationData => {
                Ok(Property::CorrelationData(codec::read_binary_data(reader)?))
            }
            PropertyId::SubscriptionIdentifier => {
                let v = codec::read_variable_byte_integer(reader)?;
                if v == 0 {
                    Err(Error::ProtocolError)
                } else {
                    Ok(Property::SubscriptionIdentifier(v))
                }
            }

            PropertyId::SessionExpiryInterval => Ok(Property::SessionExpiryInterval(
                codec::read_four_byte_integer(reader)?,
            )),
            PropertyId::AssignedClientIdentifier => Ok(Property::AssignedClientIdentifier(
                codec::read_utf8_string(reader)?,
            )),
            PropertyId::ServerKeepAlive => Ok(Property::ServerKeepAlive(
                codec::read_two_byte_integer(reader)?,
            )),
            PropertyId::AuthenticationMethod => Ok(Property::AuthenticationMethod(
                codec::read_utf8_string(reader)?,
            )),
            PropertyId::AuthenticationData => Ok(Property::AuthenticationData(
                codec::read_binary_data(reader)?,
            )),
            PropertyId::RequestProblemInformation => match codec::read_byte(reader)? {
                0x00 => Ok(Property::RequestProblemInformation(false)),
                0x01 => Ok(Property::RequestProblemInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::WillDelayInterval => Ok(Property::WillDelayInterval(
                codec::read_four_byte_integer(reader)?,
            )),
            PropertyId::RequestResponseInformation => match codec::read_byte(reader)? {
                0x00 => Ok(Property::RequestResponseInformation(false)),
                0x01 => Ok(Property::RequestResponseInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::ResponseInformation => Ok(Property::ResponseInformation(
                codec::read_utf8_string(reader)?,
            )),
            PropertyId::ServerReference => {
                Ok(Property::ServerReference(codec::read_utf8_string(reader)?))
            }
            PropertyId::ReasonString => {
                Ok(Property::ReasonString(codec::read_utf8_string(reader)?))
            }
            PropertyId::ReceiveMaximum => match codec::read_two_byte_integer(reader)? {
                0 => Err(Error::MalformedPacket),
                v => Ok(Property::ReceiveMaximum(v)),
            },
            PropertyId::TopicAliasMaximum => Ok(Property::TopicAliasMaximum(
                codec::read_two_byte_integer(reader)?,
            )),
            PropertyId::TopicAlias => {
                Ok(Property::TopicAlias(codec::read_two_byte_integer(reader)?))
            }
            PropertyId::MaximumQoS => {
                let qos = codec::read_qos(reader)?;
                if qos == QoS::ExactlyOnce {
                    Err(Error::ProtocolError)
                } else {
                    Ok(Property::MaximumQoS(qos))
                }
            }
            PropertyId::RetainAvailable => Ok(Property::RetainAvailable(codec::read_bool(reader)?)),
            PropertyId::UserProperty => Ok(Property::UserProperty(
                codec::read_utf8_string(reader)?,
                codec::read_utf8_string(reader)?,
            )),
            PropertyId::MaximumPacketSize => Ok(Property::MaximumPacketSize(
                codec::read_four_byte_integer(reader)?,
            )),
            PropertyId::WildcardSubscriptionAvailable => Ok(
                Property::WildcardSubscriptionAvailable(codec::read_bool(reader)?),
            ),
            PropertyId::SubscriptionIdentifiersAvailable => Ok(
                Property::SubscriptionIdentifiersAvailable(codec::read_bool(reader)?),
            ),
            PropertyId::SharedSubscriptionAvailable => Ok(Property::SharedSubscriptionAvailable(
                codec::read_bool(reader)?,
            )),
        }
    }
}

impl Property {
    pub fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        match self {
            Property::PayloadFormatIndicator(v) => {
                if v != DEFAULT_PAYLOAD_FORMAT_INDICATOR {
                    let n_bytes = write_property_id(PropertyId::PayloadFormatIndicator, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::MessageExpiryInterval(v) => {
                let n_bytes = write_property_id(PropertyId::MessageExpiryInterval, writer)?;
                Ok(n_bytes + codec::write_four_byte_integer(v, writer)?)
            }
            Property::ContentType(v) => {
                let n_bytes = write_property_id(PropertyId::ContentType, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::ResponseTopic(v) => {
                if v.is_empty() {
                    Err(Error::ProtocolError)
                } else {
                    let n_bytes = write_property_id(PropertyId::ResponseTopic, writer)?;
                    Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
                }
            }
            Property::CorrelationData(v) => {
                let n_bytes = write_property_id(PropertyId::CorrelationData, writer)?;
                Ok(n_bytes + codec::write_binary_data(&v, writer)?)
            }
            Property::SubscriptionIdentifier(v) => {
                if v == 0 {
                    Err(Error::ProtocolError)
                } else {
                    let n_bytes = write_property_id(PropertyId::SubscriptionIdentifier, writer)?;
                    Ok(n_bytes + codec::write_variable_byte_integer(v, writer)?)
                }
            }
            Property::SessionExpiryInterval(v) => {
                if v != DEFAULT_SESSION_EXPIRY_INTERVAL {
                    let n_bytes = write_property_id(PropertyId::SessionExpiryInterval, writer)?;
                    Ok(n_bytes + codec::write_four_byte_integer(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::AssignedClientIdentifier(v) => {
                let n_bytes = write_property_id(PropertyId::AssignedClientIdentifier, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::ServerKeepAlive(v) => {
                let n_bytes = write_property_id(PropertyId::ServerKeepAlive, writer)?;
                Ok(n_bytes + codec::write_two_byte_integer(v, writer)?)
            }
            Property::AuthenticationMethod(v) => {
                let n_bytes = write_property_id(PropertyId::AuthenticationMethod, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::AuthenticationData(v) => {
                let n_bytes = write_property_id(PropertyId::AuthenticationData, writer)?;
                Ok(n_bytes + codec::write_binary_data(&v, writer)?)
            }
            Property::RequestProblemInformation(v) => {
                if v != DEFAULT_REQUEST_PROBLEM_INFORMATION {
                    let n_bytes = write_property_id(PropertyId::RequestProblemInformation, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::WillDelayInterval(v) => {
                if v != DEFAULT_WILL_DELAY_INTERVAL {
                    let n_bytes = write_property_id(PropertyId::WillDelayInterval, writer)?;
                    Ok(n_bytes + codec::write_four_byte_integer(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::RequestResponseInformation(v) => {
                if v != DEFAULT_REQUEST_RESPONSE_INFORMATION {
                    let n_bytes =
                        write_property_id(PropertyId::RequestResponseInformation, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::ResponseInformation(v) => {
                let n_bytes = write_property_id(PropertyId::ResponseInformation, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::ServerReference(v) => {
                let n_bytes = write_property_id(PropertyId::ServerReference, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::ReasonString(v) => {
                let n_bytes = write_property_id(PropertyId::ReasonString, writer)?;
                Ok(n_bytes + codec::write_utf8_string(&v, writer)?)
            }
            Property::ReceiveMaximum(v) => match v {
                0 => Err(Error::MalformedPacket),
                DEFAULT_RECEIVE_MAXIMUM => Ok(0),
                _ => {
                    let n_bytes = write_property_id(PropertyId::ReceiveMaximum, writer)?;
                    Ok(n_bytes + codec::write_two_byte_integer(v, writer)?)
                }
            },
            Property::TopicAliasMaximum(v) => {
                if v != DEFAULT_TOPIC_ALIAS_MAXIMUM {
                    let n_bytes = write_property_id(PropertyId::TopicAliasMaximum, writer)?;
                    Ok(n_bytes + codec::write_two_byte_integer(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::TopicAlias(v) => {
                let n_bytes = write_property_id(PropertyId::TopicAlias, writer)?;
                Ok(n_bytes + codec::write_two_byte_integer(v, writer)?)
            }
            Property::MaximumQoS(v) => match v {
                DEFAULT_MAXIMUM_QOS => Ok(0),
                _ => {
                    let n_bytes = write_property_id(PropertyId::MaximumQoS, writer)?;
                    Ok(n_bytes + codec::write_qos(v, writer)?)
                }
            },
            Property::RetainAvailable(v) => {
                if v != DEFAULT_RETAIN_AVAILABLE {
                    let n_bytes = write_property_id(PropertyId::RetainAvailable, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::UserProperty(k, v) => {
                let mut n_bytes = write_property_id(PropertyId::UserProperty, writer)?;
                n_bytes += codec::write_utf8_string(&k, writer)?;
                Ok(n_bytes + (codec::write_utf8_string(&v, writer)?))
            }
            Property::MaximumPacketSize(v) => {
                let n_bytes = write_property_id(PropertyId::MaximumPacketSize, writer)?;
                Ok(n_bytes + codec::write_four_byte_integer(v, writer)?)
            }
            Property::WildcardSubscriptionAvailable(v) => {
                if v != DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE {
                    let n_bytes =
                        write_property_id(PropertyId::WildcardSubscriptionAvailable, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::SubscriptionIdentifiersAvailable(v) => {
                let n_bytes =
                    write_property_id(PropertyId::SubscriptionIdentifiersAvailable, writer)?;
                Ok(n_bytes + codec::write_bool(v, writer)?)
            }
            Property::SharedSubscriptionAvailable(v) => {
                if v != DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE {
                    let n_bytes =
                        write_property_id(PropertyId::SharedSubscriptionAvailable, writer)?;
                    Ok(n_bytes + codec::write_bool(v, writer)?)
                } else {
                    Ok(0)
                }
            }
        }
    }
}
