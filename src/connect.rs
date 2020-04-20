use crate::{
    Bits, Byte, Decode, Encode, Error, PropertiesDecoder, Property, QoS, Result as SageResult,
    TwoByteInteger, UTF8String, VariableByteInteger,
};
use std::convert::TryInto;
use std::io::{Read, Write};

/// `ConnectFlags` is a set of parameters describing the behaviour of the
/// `Connect` control packet.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ConnectFlags {
    /// Specifies wether the connection starts a new Session or is a
    /// continuation of an existing Session.
    pub clean_start: bool,
    /// Specifies wether the server must store a will message.
    pub will: bool,
    // The QoS used for the Will message
    pub will_qos: QoS,
    /// Is the Will message to be retained when it is published
    pub will_retain: bool,
    /// Specifies whether a user name is present
    pub user_name: bool,
    /// Specifies wether a password is present
    pub password: bool,
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

/// The `Connect` control packet is used to open a connection
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Connect {
    /// The control packet parameters
    pub flags: ConnectFlags,

    /// Time interval in seconds representing the maximum interval that is
    /// allowed to elapse between two client MQTT packets.
    pub keep_alive: u16,

    /// Represents the session expiry in seconds.
    pub session_expiry_interval: Option<u32>,

    /// Limits the number of QoS1 and QoS2 publications that than be processed
    /// concurrently.
    pub receive_maximum: Option<u16>,

    /// The maximum packet size the client is willing to accept
    pub maximum_packet_size: Option<u32>,

    /// Highest value a client will accept a a Topic Alias sent by the server.
    pub topic_alias_maximum: Option<u16>,

    /// Can the server send response information in the CONNACK?
    pub request_response_information: Option<bool>,

    /// Wether a reason string or user properties are sent in case of failure
    pub request_problem_information: Option<bool>,

    /// User properties can be any key-value pair. Duplicates are allowed
    pub user_properties: Vec<(String, String)>,

    /// Set if an authentication is done
    pub authentication_method: Option<String>,

    /// Sets authentication data
    pub authentication_data: Vec<u8>,
}

impl Encode for Connect {
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        // Variable Header (into content)
        let mut n_bytes = UTF8String::from("MQTT").encode(writer)?;
        n_bytes += Byte(0x05).encode(writer)?;
        n_bytes += self.flags.encode(writer)?;
        n_bytes += TwoByteInteger(self.keep_alive).encode(writer)?;

        // Properties
        let mut properties = Vec::new();
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            properties.push(Property::SessionExpiryInterval(session_expiry_interval));
        }
        if let Some(receive_maximum) = self.receive_maximum {
            properties.push(Property::ReceiveMaximum(receive_maximum));
        }
        if let Some(maximum_packet_size) = self.maximum_packet_size {
            properties.push(Property::MaximumPacketSize(maximum_packet_size));
        }
        if let Some(topic_alias_maximum) = self.topic_alias_maximum {
            properties.push(Property::TopicAliasMaximum(topic_alias_maximum));
        }
        if let Some(request_response_information) = self.request_response_information {
            properties.push(Property::RequestResponseInformation(
                request_response_information,
            ));
        }
        if let Some(request_problem_information) = self.request_problem_information {
            properties.push(Property::RequestResponseInformation(
                request_problem_information,
            ));
        }
        for property in &self.user_properties {
            properties.push(Property::UserProperty(
                property.0.clone(),
                property.1.clone(),
            ));
        }
        if let Some(authentication_method) = &self.authentication_method {
            properties.push(Property::AuthenticationMethod(
                authentication_method.clone(),
            ));
        }
        if !self.authentication_data.is_empty() {
            properties.push(Property::AuthenticationData(
                self.authentication_data.clone(),
            ));
        }

        n_bytes += {
            let mut n_bytes = 0;

            let mut properties_buffer = Vec::new();
            for property in properties {
                n_bytes += property.encode(&mut properties_buffer)?;
            }
            n_bytes += VariableByteInteger(n_bytes as u32).encode(writer)?;
            writer.write_all(&properties_buffer)?;
            n_bytes
        };

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
        let keep_alive = TwoByteInteger::decode(reader)?.into();

        let mut c = Connect {
            flags,
            keep_alive,
            ..Default::default()
        };
        let mut decoder = PropertiesDecoder::take(reader)?;
        while decoder.has_properties() {
            let p = decoder.read()?;
            match p {
                Property::SessionExpiryInterval(v) => c.session_expiry_interval = Some(v),
                Property::AuthenticationMethod(v) => c.authentication_method = Some(v),
                Property::AuthenticationData(v) => c.authentication_data = v,
                Property::RequestProblemInformation(v) => c.request_problem_information = Some(v),
                Property::RequestResponseInformation(v) => c.request_response_information = Some(v),
                Property::ReceiveMaximum(v) => c.receive_maximum = Some(v),
                Property::TopicAliasMaximum(v) => c.topic_alias_maximum = Some(v),
                Property::UserProperty(k, v) => c.user_properties.push((k, v)),
                Property::MaximumPacketSize(v) => c.maximum_packet_size = Some(v),
                _ => return Err(Error::ProtocolError),
            };
        }

        // if authentication_data.is_some() != authentication_method.is_some() {
        //     return Err(Error::ProtocolError);
        // }
        // let authentication_data = authentication_data.unwrap_or_default();

        Ok(c)
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
        //0x10, 0xFF, 0x7F, // Change the size
        0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x05, 0xCE, 0x00, 0x0A, 0x05, 0x11, 0x00, 0x00, 0x00,
        0x0A,
    ];

    fn connect_decoded() -> Connect {
        let flags = ConnectFlags {
            clean_start: true,
            will: true,
            will_qos: QoS::AtLeastOnce,
            will_retain: false,
            user_name: true,
            password: true,
        };
        let keep_alive = 10;

        let session_expiry_interval = Some(10);

        Connect {
            flags,
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
