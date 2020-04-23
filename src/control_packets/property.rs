use crate::{
    Error, PropertyId, QoS, ReadBinaryData, ReadByte, ReadFourByteInteger, ReadTwoByteInteger,
    ReadUTF8String, ReadVariableByteInteger, Result as SageResult, WriteBinaryData, WriteByte,
    WriteFourByteInteger, WriteTwoByteInteger, WriteUTF8String, WriteVariableByteInteger,
    DEFAULT_MAXIMUM_QOS, DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM,
    DEFAULT_REQUEST_PROBLEM_INFORMATION, DEFAULT_REQUEST_RESPONSE_INFORMATION,
    DEFAULT_RETAIN_AVAILABLE, DEFAULT_SESSION_EXPIRY_INTERVAL,
    DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE, DEFAULT_TOPIC_ALIAS_MAXIMUM,
    DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE, DEFAULT_WILL_DELAY_INTERVAL,
};
use std::{
    collections::HashSet,
    io::{Read, Take, Write},
};

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
    SubscriptionIdentifierAvailable(bool),
    SharedSubscriptionAvailable(bool),
}

pub struct PropertiesDecoder<'a, R: Read> {
    reader: Take<&'a mut R>,
    marked: HashSet<PropertyId>,
}

impl<'a, R: Read> PropertiesDecoder<'a, R> {
    pub fn take(reader: &'a mut R) -> SageResult<Self> {
        let len = u64::read_variable_byte_integer(reader)?;
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
        let property_id = PropertyId::read_variable_byte_integer(reader)?;

        // Filter by authorized properties and unicity requirements
        if property_id != PropertyId::UserProperty && !self.marked.insert(property_id) {
            return Err(Error::ProtocolError);
        }
        self.read_property_value(property_id)
    }

    fn read_property_value(&mut self, id: PropertyId) -> SageResult<Property> {
        let reader = &mut self.reader;
        match id {
            PropertyId::PayloadFormatIndicator => match u8::read_byte(reader)? {
                0x00 => Ok(Property::PayloadFormatIndicator(false)),
                0x01 => Ok(Property::PayloadFormatIndicator(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::MessageExpiryInterval => Ok(Property::MessageExpiryInterval(
                u32::read_four_byte_integer(reader)?,
            )),
            PropertyId::ContentType => Ok(Property::ContentType(String::read_utf8_string(reader)?)),
            PropertyId::ResponseTopic => {
                Ok(Property::ResponseTopic(String::read_utf8_string(reader)?))
            }
            PropertyId::CorrelationData => {
                Ok(Property::CorrelationData(Vec::read_binary_data(reader)?))
            }
            PropertyId::SubscriptionIdentifier => {
                let v = u32::read_variable_byte_integer(reader)?;
                if v == 0 {
                    Err(Error::ProtocolError)
                } else {
                    Ok(Property::SubscriptionIdentifier(v))
                }
            }

            PropertyId::SessionExpiryInterval => Ok(Property::SessionExpiryInterval(
                u32::read_four_byte_integer(reader)?,
            )),
            PropertyId::AssignedClientIdentifier => Ok(Property::AssignedClientIdentifier(
                String::read_utf8_string(reader)?,
            )),
            PropertyId::ServerKeepAlive => Ok(Property::ServerKeepAlive(
                u16::read_two_byte_integer(reader)?,
            )),
            PropertyId::AuthenticationMethod => Ok(Property::AuthenticationMethod(
                String::read_utf8_string(reader)?,
            )),
            PropertyId::AuthenticationData => {
                Ok(Property::AuthenticationData(Vec::read_binary_data(reader)?))
            }
            PropertyId::RequestProblemInformation => match u8::read_byte(reader)? {
                0x00 => Ok(Property::RequestProblemInformation(false)),
                0x01 => Ok(Property::RequestProblemInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::WillDelayInterval => Ok(Property::WillDelayInterval(
                u32::read_four_byte_integer(reader)?,
            )),
            PropertyId::RequestResponseInformation => match u8::read_byte(reader)? {
                0x00 => Ok(Property::RequestResponseInformation(false)),
                0x01 => Ok(Property::RequestResponseInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::ResponseInformation => Ok(Property::ResponseInformation(
                String::read_utf8_string(reader)?,
            )),
            PropertyId::ServerReference => {
                Ok(Property::ServerReference(String::read_utf8_string(reader)?))
            }
            PropertyId::ReasonString => {
                Ok(Property::ReasonString(String::read_utf8_string(reader)?))
            }
            PropertyId::ReceiveMaximum => match u16::read_two_byte_integer(reader)? {
                0 => Err(Error::MalformedPacket),
                v => Ok(Property::ReceiveMaximum(v)),
            },
            PropertyId::TopicAliasMaximum => Ok(Property::TopicAliasMaximum(
                u16::read_two_byte_integer(reader)?,
            )),
            PropertyId::TopicAlias => Ok(Property::TopicAlias(u16::read_two_byte_integer(reader)?)),
            PropertyId::MaximumQoS => Ok(Property::MaximumQoS(QoS::read_byte(reader)?)),
            PropertyId::RetainAvailable => Ok(Property::RetainAvailable(bool::read_byte(reader)?)),
            PropertyId::UserProperty => Ok(Property::UserProperty(
                String::read_utf8_string(reader)?,
                String::read_utf8_string(reader)?,
            )),
            PropertyId::MaximumPacketSize => Ok(Property::MaximumPacketSize(
                u32::read_four_byte_integer(reader)?,
            )),
            PropertyId::WildcardSubscriptionAvailable => Ok(
                Property::WildcardSubscriptionAvailable(bool::read_byte(reader)?),
            ),
            PropertyId::SubscriptionIdentifierAvailable => Ok(
                Property::SubscriptionIdentifierAvailable(bool::read_byte(reader)?),
            ),
            PropertyId::SharedSubscriptionAvailable => Ok(Property::SharedSubscriptionAvailable(
                bool::read_byte(reader)?,
            )),
        }
    }
}

impl Property {
    pub fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        match self {
            Property::PayloadFormatIndicator(v) => {
                if v != DEFAULT_PAYLOAD_FORMAT_INDICATOR {
                    let n_bytes =
                        PropertyId::PayloadFormatIndicator.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::MessageExpiryInterval(v) => {
                let n_bytes =
                    PropertyId::MessageExpiryInterval.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_four_byte_integer(writer)?)
            }
            Property::ContentType(v) => {
                let n_bytes = PropertyId::ContentType.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::ResponseTopic(v) => {
                let n_bytes = PropertyId::ResponseTopic.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::CorrelationData(v) => {
                let n_bytes = PropertyId::CorrelationData.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_binary_data(writer)?)
            }
            Property::SubscriptionIdentifier(v) => {
                if v == 0 {
                    Err(Error::ProtocolError)
                } else {
                    let n_bytes =
                        PropertyId::SubscriptionIdentifier.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_variable_byte_integer(writer)?)
                }
            }
            Property::SessionExpiryInterval(v) => {
                if v != DEFAULT_SESSION_EXPIRY_INTERVAL {
                    let n_bytes =
                        PropertyId::SessionExpiryInterval.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_four_byte_integer(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::AssignedClientIdentifier(v) => {
                let n_bytes =
                    PropertyId::AssignedClientIdentifier.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::ServerKeepAlive(v) => {
                let n_bytes = PropertyId::ServerKeepAlive.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_two_byte_integer(writer)?)
            }
            Property::AuthenticationMethod(v) => {
                let n_bytes =
                    PropertyId::AuthenticationMethod.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::AuthenticationData(v) => {
                let n_bytes = PropertyId::AuthenticationData.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_binary_data(writer)?)
            }
            Property::RequestProblemInformation(v) => {
                if v != DEFAULT_REQUEST_PROBLEM_INFORMATION {
                    let n_bytes = PropertyId::RequestProblemInformation
                        .write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::WillDelayInterval(v) => {
                if v != DEFAULT_WILL_DELAY_INTERVAL {
                    let n_bytes =
                        PropertyId::WillDelayInterval.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_four_byte_integer(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::RequestResponseInformation(v) => {
                if v != DEFAULT_REQUEST_RESPONSE_INFORMATION {
                    let n_bytes = PropertyId::RequestResponseInformation
                        .write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::ResponseInformation(v) => {
                let n_bytes =
                    PropertyId::ResponseInformation.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::ServerReference(v) => {
                let n_bytes = PropertyId::ServerReference.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::ReasonString(v) => {
                let n_bytes = PropertyId::ReasonString.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_utf8_string(writer)?)
            }
            Property::ReceiveMaximum(v) => match v {
                0 => Err(Error::MalformedPacket),
                DEFAULT_RECEIVE_MAXIMUM => Ok(0),
                _ => {
                    let n_bytes = PropertyId::ReceiveMaximum.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_two_byte_integer(writer)?)
                }
            },
            Property::TopicAliasMaximum(v) => {
                if v != DEFAULT_TOPIC_ALIAS_MAXIMUM {
                    let n_bytes =
                        PropertyId::TopicAliasMaximum.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_two_byte_integer(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::TopicAlias(v) => {
                let n_bytes = PropertyId::TopicAlias.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_two_byte_integer(writer)?)
            }
            Property::MaximumQoS(v) => {
                if v != DEFAULT_MAXIMUM_QOS {
                    let n_bytes = PropertyId::MaximumQoS.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::RetainAvailable(v) => {
                if v != DEFAULT_RETAIN_AVAILABLE {
                    let n_bytes =
                        PropertyId::RetainAvailable.write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::UserProperty(k, v) => {
                let mut n_bytes = PropertyId::UserProperty.write_variable_byte_integer(writer)?;
                n_bytes += k.write_utf8_string(writer)?;
                Ok(n_bytes + (v.write_utf8_string(writer)?))
            }
            Property::MaximumPacketSize(v) => {
                let n_bytes = PropertyId::MaximumPacketSize.write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_four_byte_integer(writer)?)
            }
            Property::WildcardSubscriptionAvailable(v) => {
                if v != DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE {
                    let n_bytes = PropertyId::WildcardSubscriptionAvailable
                        .write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::SubscriptionIdentifierAvailable(v) => {
                let n_bytes = PropertyId::SubscriptionIdentifierAvailable
                    .write_variable_byte_integer(writer)?;
                Ok(n_bytes + v.write_byte(writer)?)
            }
            Property::SharedSubscriptionAvailable(v) => {
                if v != DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE {
                    let n_bytes = PropertyId::SharedSubscriptionAvailable
                        .write_variable_byte_integer(writer)?;
                    Ok(n_bytes + v.write_byte(writer)?)
                } else {
                    Ok(0)
                }
            }
        }
    }
}
