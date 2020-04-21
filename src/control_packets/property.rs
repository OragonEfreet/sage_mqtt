use crate::{
    BinaryData, Bits, Byte, Decode, Encode, Error, FourByteInteger, PropertyId, QoS,
    Result as SageResult, TwoByteInteger, UTF8String, VariableByteInteger, DEFAULT_MAXIMUM_QOS,
    DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM, DEFAULT_REQUEST_PROBLEM_INFORMATION,
    DEFAULT_REQUEST_RESPONSE_INFORMATION, DEFAULT_RETAIN_AVAILABLE,
    DEFAULT_SESSION_EXPIRY_INTERVAL, DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE,
    DEFAULT_WILL_DELAY_INTERVAL,
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
        let len = u64::from(VariableByteInteger::decode(reader)?);
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
        let property_id = PropertyId::decode(reader)?;

        // Filter by authorized properties and unicity requirements
        if property_id != PropertyId::UserProperty && !self.marked.insert(property_id) {
            return Err(Error::ProtocolError);
        }
        self.read_property_value(property_id)
    }

    fn read_property_value(&mut self, id: PropertyId) -> SageResult<Property> {
        let reader = &mut self.reader;
        match id {
            PropertyId::PayloadFormatIndicator => match u8::from(Byte::decode(reader)?) {
                0x00 => Ok(Property::PayloadFormatIndicator(false)),
                0x01 => Ok(Property::PayloadFormatIndicator(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::MessageExpiryInterval => Ok(Property::MessageExpiryInterval(
                FourByteInteger::decode(reader)?.into(),
            )),
            PropertyId::ContentType => {
                Ok(Property::ContentType(UTF8String::decode(reader)?.into()))
            }
            PropertyId::ResponseTopic => {
                Ok(Property::ResponseTopic(UTF8String::decode(reader)?.into()))
            }
            PropertyId::CorrelationData => Ok(Property::CorrelationData(
                BinaryData::decode(reader)?.into(),
            )),
            PropertyId::SubscriptionIdentifier => Ok(Property::SubscriptionIdentifier(
                VariableByteInteger::decode(reader)?.into(),
            )),
            PropertyId::SessionExpiryInterval => Ok(Property::SessionExpiryInterval(
                FourByteInteger::decode(reader)?.into(),
            )),
            PropertyId::AssignedClientIdentifier => Ok(Property::AssignedClientIdentifier(
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::ServerKeepAlive => Ok(Property::ServerKeepAlive(
                TwoByteInteger::decode(reader)?.into(),
            )),
            PropertyId::AuthenticationMethod => Ok(Property::AuthenticationMethod(
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::AuthenticationData => Ok(Property::AuthenticationData(
                BinaryData::decode(reader)?.into(),
            )),
            PropertyId::RequestProblemInformation => match u8::from(Byte::decode(reader)?) {
                0x00 => Ok(Property::RequestProblemInformation(false)),
                0x01 => Ok(Property::RequestProblemInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::WillDelayInterval => Ok(Property::WillDelayInterval(
                FourByteInteger::decode(reader)?.into(),
            )),
            PropertyId::RequestResponseInformation => match u8::from(Byte::decode(reader)?) {
                0x00 => Ok(Property::RequestResponseInformation(false)),
                0x01 => Ok(Property::RequestResponseInformation(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::ResponseInformation => Ok(Property::ResponseInformation(
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::ServerReference => Ok(Property::ServerReference(
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::ReasonString => {
                Ok(Property::ReasonString(UTF8String::decode(reader)?.into()))
            }
            PropertyId::ReceiveMaximum => match u16::from(TwoByteInteger::decode(reader)?) {
                0 => Err(Error::MalformedPacket),
                v => Ok(Property::ReceiveMaximum(v)),
            },
            PropertyId::TopicAliasMaximum => Ok(Property::TopicAliasMaximum(
                TwoByteInteger::decode(reader)?.into(),
            )),
            PropertyId::TopicAlias => {
                Ok(Property::TopicAlias(TwoByteInteger::decode(reader)?.into()))
            }
            PropertyId::MaximumQoS => Ok(Property::MaximumQoS(QoS::decode(reader)?)),
            PropertyId::RetainAvailable => match Byte::decode(reader)? {
                Byte(0x00) => Ok(Property::RetainAvailable(false)),
                Byte(0x01) => Ok(Property::RetainAvailable(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::UserProperty => Ok(Property::UserProperty(
                UTF8String::decode(reader)?.into(),
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::MaximumPacketSize => Ok(Property::MaximumPacketSize(
                FourByteInteger::decode(reader)?.into(),
            )),
            PropertyId::WildcardSubscriptionAvailable => match Byte::decode(reader)? {
                Byte(0x00) => Ok(Property::WildcardSubscriptionAvailable(false)),
                Byte(0x01) => Ok(Property::WildcardSubscriptionAvailable(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::SubscriptionIdentifierAvailable => match Byte::decode(reader)? {
                Byte(0x00) => Ok(Property::SubscriptionIdentifierAvailable(false)),
                Byte(0x01) => Ok(Property::SubscriptionIdentifierAvailable(true)),
                _ => Err(Error::ProtocolError),
            },
            PropertyId::SharedSubscriptionAvailable => match Byte::decode(reader)? {
                Byte(0x00) => Ok(Property::SharedSubscriptionAvailable(false)),
                Byte(0x01) => Ok(Property::SharedSubscriptionAvailable(true)),
                _ => Err(Error::ProtocolError),
            },
        }
    }
}

impl Encode for Property {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        match self {
            Property::PayloadFormatIndicator(v) => {
                if v != DEFAULT_PAYLOAD_FORMAT_INDICATOR {
                    let n_bytes = PropertyId::PayloadFormatIndicator.encode(writer)?;
                    Ok(n_bytes + Bits(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::MessageExpiryInterval(v) => {
                let n_bytes = PropertyId::MessageExpiryInterval.encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::ContentType(v) => {
                let n_bytes = PropertyId::ContentType.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ResponseTopic(v) => {
                let n_bytes = PropertyId::ResponseTopic.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::CorrelationData(v) => {
                let n_bytes = PropertyId::CorrelationData.encode(writer)?;
                Ok(n_bytes + BinaryData(v).encode(writer)?)
            }
            Property::SubscriptionIdentifier(v) => {
                let n_bytes = PropertyId::SubscriptionIdentifier.encode(writer)?;
                Ok(n_bytes + VariableByteInteger(v).encode(writer)?)
            }
            Property::SessionExpiryInterval(v) => {
                if v != DEFAULT_SESSION_EXPIRY_INTERVAL {
                    let n_bytes = PropertyId::SessionExpiryInterval.encode(writer)?;
                    Ok(n_bytes + FourByteInteger(v).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::AssignedClientIdentifier(v) => {
                let n_bytes = PropertyId::AssignedClientIdentifier.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ServerKeepAlive(v) => {
                let n_bytes = PropertyId::ServerKeepAlive.encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::AuthenticationMethod(v) => {
                let n_bytes = PropertyId::AuthenticationMethod.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::AuthenticationData(v) => {
                let n_bytes = PropertyId::AuthenticationData.encode(writer)?;
                Ok(n_bytes + BinaryData(v).encode(writer)?)
            }
            Property::RequestProblemInformation(v) => {
                if v != DEFAULT_REQUEST_PROBLEM_INFORMATION {
                    let n_bytes = PropertyId::RequestProblemInformation.encode(writer)?;
                    Ok(n_bytes + Byte(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::WillDelayInterval(v) => {
                if v != DEFAULT_WILL_DELAY_INTERVAL {
                    let n_bytes = PropertyId::WillDelayInterval.encode(writer)?;
                    Ok(n_bytes + FourByteInteger(v).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::RequestResponseInformation(v) => {
                if v != DEFAULT_REQUEST_RESPONSE_INFORMATION {
                    let n_bytes = PropertyId::RequestResponseInformation.encode(writer)?;
                    Ok(n_bytes + Byte(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::ResponseInformation(v) => {
                let n_bytes = PropertyId::ResponseInformation.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ServerReference(v) => {
                let n_bytes = PropertyId::ServerReference.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ReasonString(v) => {
                let n_bytes = PropertyId::ReasonString.encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ReceiveMaximum(v) => match v {
                0 => Err(Error::MalformedPacket),
                DEFAULT_RECEIVE_MAXIMUM => Ok(0),
                _ => {
                    let n_bytes = PropertyId::ReceiveMaximum.encode(writer)?;
                    Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
                }
            },
            Property::TopicAliasMaximum(v) => {
                if v != DEFAULT_TOPIC_ALIAS_MAXIMUM {
                    let n_bytes = PropertyId::TopicAliasMaximum.encode(writer)?;
                    Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::TopicAlias(v) => {
                let n_bytes = PropertyId::TopicAlias.encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::MaximumQoS(v) => {
                if v != DEFAULT_MAXIMUM_QOS {
                    let n_bytes = PropertyId::MaximumQoS.encode(writer)?;
                    Ok(n_bytes + v.encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::RetainAvailable(v) => {
                if v != DEFAULT_RETAIN_AVAILABLE {
                    let n_bytes = PropertyId::RetainAvailable.encode(writer)?;
                    Ok(n_bytes + Byte(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::UserProperty(k, v) => {
                let mut n_bytes = PropertyId::UserProperty.encode(writer)?;
                n_bytes += UTF8String(k).encode(writer)?;
                Ok(n_bytes + (UTF8String(v).encode(writer)?))
            }
            Property::MaximumPacketSize(v) => {
                let n_bytes = PropertyId::MaximumPacketSize.encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::WildcardSubscriptionAvailable(v) => {
                if v != DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE {
                    let n_bytes = PropertyId::WildcardSubscriptionAvailable.encode(writer)?;
                    Ok(n_bytes + Byte(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
            Property::SubscriptionIdentifierAvailable(v) => {
                let n_bytes = PropertyId::SubscriptionIdentifierAvailable.encode(writer)?;
                Ok(n_bytes + Byte(v as u8).encode(writer)?)
            }
            Property::SharedSubscriptionAvailable(v) => {
                if v != DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE {
                    let n_bytes = PropertyId::SharedSubscriptionAvailable.encode(writer)?;
                    Ok(n_bytes + Byte(v as u8).encode(writer)?)
                } else {
                    Ok(0)
                }
            }
        }
    }
}
