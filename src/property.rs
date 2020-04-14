use crate::{
    BinaryData, Bits, Byte, Decode, Encode, Error, FourByteInteger, PropertyId,
    Result as SageResult, TwoByteInteger, UTF8String, VariableByteInteger,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
pub enum Property {
    PayloadFormatIndicator(u8),
    MessageExpiryInterval(u32),
    ContentType(String),
    ResponseTopic(String),
    CorrelationData(Vec<u8>),
    SubscriptionIdentifier(u32),
    SessionExpiryInterval(u32),
    AssignedClientIdentifier(String),
    ServerKeepAlive(u16),
    Authenticationmethod(String),
    AuthenticationData(Vec<u8>),
    RequestProblemInformation(u8),
    WillDelayInterval(u32),
    RequestResponseInformation(u8),
    ResponseInformation(String),
    ServerReference(String),
    ReasonString(String),
    ReceiveMaximum(u16),
    TopicAliasMaximum(u16),
    TopicAlias(u16),
    MaximumQoS(u8),
    RetainAvailable(u8),
    UserProperty((String, String)),
    MaximumPacketSize(u32),
    WildcardSubscriptionAvailable(u8),
    SubscriptionIdentifierAvailable(u8),
    SharedSubscriptionAvailable(u8),
}

impl Encode for Property {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        match &self {
            Property::PayloadFormatIndicator(v) => {
                let n_bytes = VariableByteInteger(PropertyId::PayloadFormatIndicator as u32)
                    .encode(writer)?;
                Ok(n_bytes + Bits(*v).encode(writer)?)
            }
            Property::MessageExpiryInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::MessageExpiryInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(*v).encode(writer)?)
            }
            Property::ContentType(v) => {
                let n_bytes = VariableByteInteger(PropertyId::ContentType as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::ResponseTopic(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ResponseTopic as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::CorrelationData(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::CorrelationData as u32).encode(writer)?;
                Ok(n_bytes + BinaryData(v.clone()).encode(writer)?)
            }
            Property::SubscriptionIdentifier(v) => {
                let n_bytes = VariableByteInteger(PropertyId::SubscriptionIdentifier as u32)
                    .encode(writer)?;
                Ok(n_bytes + VariableByteInteger(*v).encode(writer)?)
            }
            Property::SessionExpiryInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::SessionExpiryInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(*v).encode(writer)?)
            }
            Property::AssignedClientIdentifier(v) => {
                let n_bytes = VariableByteInteger(PropertyId::AssignedClientIdentifier as u32)
                    .encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::ServerKeepAlive(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ServerKeepAlive as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(*v).encode(writer)?)
            }
            Property::Authenticationmethod(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::Authenticationmethod as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::AuthenticationData(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::AuthenticationData as u32).encode(writer)?;
                Ok(n_bytes + BinaryData(v.clone()).encode(writer)?)
            }
            Property::RequestProblemInformation(v) => {
                let n_bytes = VariableByteInteger(PropertyId::RequestProblemInformation as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::WillDelayInterval(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::WillDelayInterval as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(*v).encode(writer)?)
            }
            Property::RequestResponseInformation(v) => {
                let n_bytes = VariableByteInteger(PropertyId::RequestResponseInformation as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::ResponseInformation(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ResponseInformation as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::ServerReference(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ServerReference as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::ReasonString(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ReasonString as u32).encode(writer)?;
                Ok(n_bytes + UTF8String(v.clone()).encode(writer)?)
            }
            Property::ReceiveMaximum(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::ReceiveMaximum as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(*v).encode(writer)?)
            }
            Property::TopicAliasMaximum(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::TopicAliasMaximum as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(*v).encode(writer)?)
            }
            Property::TopicAlias(v) => {
                let n_bytes = VariableByteInteger(PropertyId::TopicAlias as u32).encode(writer)?;
                Ok(n_bytes + TwoByteInteger(*v).encode(writer)?)
            }
            Property::MaximumQoS(v) => {
                let n_bytes = VariableByteInteger(PropertyId::MaximumQoS as u32).encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::RetainAvailable(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::RetainAvailable as u32).encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::UserProperty(v) => {
                let mut n_bytes =
                    VariableByteInteger(PropertyId::UserProperty as u32).encode(writer)?;
                n_bytes += UTF8String(v.0.clone()).encode(writer)?;
                Ok(n_bytes + (UTF8String(v.1.clone()).encode(writer)?))
            }
            Property::MaximumPacketSize(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::MaximumPacketSize as u32).encode(writer)?;
                Ok(n_bytes + FourByteInteger(*v).encode(writer)?)
            }
            Property::WildcardSubscriptionAvailable(v) => {
                let n_bytes = VariableByteInteger(PropertyId::WildcardSubscriptionAvailable as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::SubscriptionIdentifierAvailable(v) => {
                let n_bytes =
                    VariableByteInteger(PropertyId::SubscriptionIdentifierAvailable as u32)
                        .encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
            Property::SharedSubscriptionAvailable(v) => {
                let n_bytes = VariableByteInteger(PropertyId::SharedSubscriptionAvailable as u32)
                    .encode(writer)?;
                Ok(n_bytes + Byte(*v).encode(writer)?)
            }
        }
    }
}

impl Decode for Property {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let property_id: Option<PropertyId> = VariableByteInteger::decode(reader)?.into();
        if let Some(property_id) = property_id {
            let property = match property_id {
                PropertyId::PayloadFormatIndicator => {
                    Property::PayloadFormatIndicator(Bits::decode(reader)?.into())
                }
                PropertyId::MessageExpiryInterval => {
                    Property::MessageExpiryInterval(FourByteInteger::decode(reader)?.into())
                }
                PropertyId::ContentType => {
                    Property::ContentType(UTF8String::decode(reader)?.into())
                }
                PropertyId::ResponseTopic => {
                    Property::ResponseTopic(UTF8String::decode(reader)?.into())
                }
                PropertyId::CorrelationData => {
                    Property::CorrelationData(BinaryData::decode(reader)?.into())
                }
                PropertyId::SubscriptionIdentifier => {
                    Property::SubscriptionIdentifier(VariableByteInteger::decode(reader)?.into())
                }
                PropertyId::SessionExpiryInterval => {
                    Property::SessionExpiryInterval(FourByteInteger::decode(reader)?.into())
                }
                PropertyId::AssignedClientIdentifier => {
                    Property::AssignedClientIdentifier(UTF8String::decode(reader)?.into())
                }
                PropertyId::ServerKeepAlive => {
                    Property::ServerKeepAlive(TwoByteInteger::decode(reader)?.into())
                }
                PropertyId::Authenticationmethod => {
                    Property::Authenticationmethod(UTF8String::decode(reader)?.into())
                }
                PropertyId::AuthenticationData => {
                    Property::AuthenticationData(BinaryData::decode(reader)?.into())
                }
                PropertyId::RequestProblemInformation => {
                    Property::RequestProblemInformation(Byte::decode(reader)?.into())
                }
                PropertyId::WillDelayInterval => {
                    Property::WillDelayInterval(FourByteInteger::decode(reader)?.into())
                }
                PropertyId::RequestResponseInformation => {
                    Property::RequestResponseInformation(Byte::decode(reader)?.into())
                }
                PropertyId::ResponseInformation => {
                    Property::ResponseInformation(UTF8String::decode(reader)?.into())
                }
                PropertyId::ServerReference => {
                    Property::ServerReference(UTF8String::decode(reader)?.into())
                }
                PropertyId::ReasonString => {
                    Property::ReasonString(UTF8String::decode(reader)?.into())
                }
                PropertyId::ReceiveMaximum => {
                    Property::ReceiveMaximum(TwoByteInteger::decode(reader)?.into())
                }
                PropertyId::TopicAliasMaximum => {
                    Property::TopicAliasMaximum(TwoByteInteger::decode(reader)?.into())
                }
                PropertyId::TopicAlias => {
                    Property::TopicAlias(TwoByteInteger::decode(reader)?.into())
                }
                PropertyId::MaximumQoS => Property::MaximumQoS(Byte::decode(reader)?.into()),
                PropertyId::RetainAvailable => {
                    Property::RetainAvailable(Byte::decode(reader)?.into())
                }
                PropertyId::UserProperty => Property::UserProperty((
                    UTF8String::decode(reader)?.into(),
                    UTF8String::decode(reader)?.into(),
                )),
                PropertyId::MaximumPacketSize => {
                    Property::MaximumPacketSize(FourByteInteger::decode(reader)?.into())
                }
                PropertyId::WildcardSubscriptionAvailable => {
                    Property::WildcardSubscriptionAvailable(Byte::decode(reader)?.into())
                }
                PropertyId::SubscriptionIdentifierAvailable => {
                    Property::SubscriptionIdentifierAvailable(Byte::decode(reader)?.into())
                }
                PropertyId::SharedSubscriptionAvailable => {
                    Property::SharedSubscriptionAvailable(Byte::decode(reader)?.into())
                }
            };
            Ok(property)
        } else {
            Err(Error::MalformedPacket)
        }
    }
}

impl Encode for Vec<Property> {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = 0;
        let mut buffer = Vec::new();
        for property in self {
            n_bytes += property.encode(&mut buffer)?;
        }
        n_bytes += VariableByteInteger(n_bytes as u32).encode(writer)?;
        writer.write_all(&buffer)?;
        Ok(n_bytes)
    }
}

impl Decode for Vec<Property> {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let mut properties = Vec::new();

        let len: u64 = VariableByteInteger::decode(reader)?.into();
        let mut buffer = reader.take(len);

        while let Ok(property) = Property::decode(&mut buffer) {
            properties.push(property);
        }

        Ok(properties)
    }
}
