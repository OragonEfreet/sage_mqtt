use crate::{
    BinaryData, Bits, Byte, Decode, Encode, Error, FourByteInteger, PropertyId,
    Result as SageResult, TwoByteInteger, UTF8String, VariableByteInteger,
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
    MaximumQoS(u8),
    RetainAvailable(u8),
    UserProperty(String, String),
    MaximumPacketSize(u32),
    WildcardSubscriptionAvailable(u8),
    SubscriptionIdentifierAvailable(u8),
    SharedSubscriptionAvailable(u8),
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
            PropertyId::ReceiveMaximum => Ok(Property::ReceiveMaximum(
                TwoByteInteger::decode(reader)?.into(),
            )),
            PropertyId::TopicAliasMaximum => Ok(Property::TopicAliasMaximum(
                TwoByteInteger::decode(reader)?.into(),
            )),
            PropertyId::TopicAlias => {
                Ok(Property::TopicAlias(TwoByteInteger::decode(reader)?.into()))
            }
            PropertyId::MaximumQoS => Ok(Property::MaximumQoS(Byte::decode(reader)?.into())),
            PropertyId::RetainAvailable => {
                Ok(Property::RetainAvailable(Byte::decode(reader)?.into()))
            }
            PropertyId::UserProperty => Ok(Property::UserProperty(
                UTF8String::decode(reader)?.into(),
                UTF8String::decode(reader)?.into(),
            )),
            PropertyId::MaximumPacketSize => Ok(Property::MaximumPacketSize(
                FourByteInteger::decode(reader)?.into(),
            )),
            PropertyId::WildcardSubscriptionAvailable => Ok(
                Property::WildcardSubscriptionAvailable(Byte::decode(reader)?.into()),
            ),
            PropertyId::SubscriptionIdentifierAvailable => Ok(
                Property::SubscriptionIdentifierAvailable(Byte::decode(reader)?.into()),
            ),
            PropertyId::SharedSubscriptionAvailable => Ok(Property::SharedSubscriptionAvailable(
                Byte::decode(reader)?.into(),
            )),
        }
    }
}

impl Encode for Property {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        match self {
            Property::PayloadFormatIndicator(v) => {
                let n_bytes = VariableByteInteger(PropertyId::PayloadFormatIndicator as u32)
                    .encode(writer)?;
                Ok(n_bytes + Bits(v as u8).encode(writer)?)
            }
            Property::MessageExpiryInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::MessageExpiryInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::ContentType(v) => {
                let n_bytes = VariableByteInteger(PropertyId::ContentType as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ResponseTopic(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ResponseTopic as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::CorrelationData(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::CorrelationData as u32).encode(writer)?;
                Ok(n_bytes + BinaryData(v).encode(writer)?)
            }
            Property::SubscriptionIdentifier(v) => {
                let n_bytes = VariableByteInteger(PropertyId::SubscriptionIdentifier as u32)
                    .encode(writer)?;
                Ok(n_bytes + VariableByteInteger(v).encode(writer)?)
            }
            Property::SessionExpiryInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::SessionExpiryInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::AssignedClientIdentifier(v) => {
                let n_bytes = VariableByteInteger(PropertyId::AssignedClientIdentifier as u32)
                    .encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ServerKeepAlive(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ServerKeepAlive as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::AuthenticationMethod(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::AuthenticationMethod as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::AuthenticationData(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::AuthenticationData as u32).encode(writer)?;
                Ok(n_bytes + BinaryData(v).encode(writer)?)
            }
            Property::RequestProblemInformation(v) => {
                let n_bytes = VariableByteInteger(PropertyId::RequestProblemInformation as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(v as u8).encode(writer)?)
            }
            Property::WillDelayInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::WillDelayInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::RequestResponseInformation(v) => {
                let n_bytes = VariableByteInteger(PropertyId::RequestResponseInformation as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(v as u8).encode(writer)?)
            }
            Property::ResponseInformation(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ResponseInformation as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ServerReference(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ServerReference as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ReasonString(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ReasonString as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v).encode(writer)?)
            }
            Property::ReceiveMaximum(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ReceiveMaximum as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::TopicAliasMaximum(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::TopicAliasMaximum as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::TopicAlias(v) => {
                let n_bytes = VariableByteInteger(PropertyId::TopicAlias as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(v).encode(writer)?)
            }
            Property::MaximumQoS(v) => {
                let n_bytes = VariableByteInteger(PropertyId::MaximumQoS as u32).encode(writer)?;
                Ok(n_bytes + Byte(v).encode(writer)?)
            }
            Property::RetainAvailable(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::RetainAvailable as u32).encode(writer)?;
                Ok(n_bytes + Byte(v).encode(writer)?)
            }
            Property::UserProperty(k, v) => {
                let mut n_bytes =
                    VariableByteInteger(PropertyId::UserProperty as u32).encode(writer)?;
                n_bytes += UTF8String(k).encode(writer)?;
                Ok(n_bytes + (UTF8String(v).encode(writer)?))
            }
            Property::MaximumPacketSize(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::MaximumPacketSize as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(v).encode(writer)?)
            }
            Property::WildcardSubscriptionAvailable(v) => {
                let n_bytes = VariableByteInteger(PropertyId::WildcardSubscriptionAvailable as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(v).encode(writer)?)
            }
            Property::SubscriptionIdentifierAvailable(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::SubscriptionIdentifierAvailable as u32)
                        .encode(writer)?;
                Ok(n_bytes + Byte(v).encode(writer)?)
            }
            Property::SharedSubscriptionAvailable(v) => {
                let n_bytes = VariableByteInteger(PropertyId::SharedSubscriptionAvailable as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(v).encode(writer)?)
            }
        }
    }
}
