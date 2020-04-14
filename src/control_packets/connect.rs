use crate::{
    Bits, Byte, ControlPacketType, Decode, Encode, Error, FixedHeader, Property, QoS,
    Result as SageResult, TwoByteInteger, UTF8String,
};
use std::convert::TryInto;
use std::io::{Read, Write};

/// `ConnectFlags` is a set of parameters describing the behaviour of the
/// `Connect` control packet.
#[derive(Default, Debug, PartialEq)]
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
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
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
#[derive(Default, PartialEq, Debug)]
pub struct Connect {
    /// The control packet parameters
    pub flags: ConnectFlags,

    /// Time interval in seconds representing the maximum interval that is
    /// allowed to elapse between two client MQTT packets.
    pub keep_alive: u16,

    pub properties: Vec<Property>,
}

impl Encode for Connect {
    fn encode<W: Write>(&self, writer: &mut W) -> SageResult<usize> {
        let mut content = Vec::new();

        // Variable Header (into content)
        let mut n_bytes = UTF8String::from("MQTT").encode(&mut content)?;
        n_bytes += Byte(0x05).encode(&mut content)?;
        n_bytes += self.flags.encode(&mut content)?;
        n_bytes += TwoByteInteger(self.keep_alive).encode(&mut content)?;

        n_bytes += self.properties.encode(&mut content)?;

        let packet_type = ControlPacketType::CONNECT;
        let remaining_size = 16_383; // TODO: change to content.len() as u32;

        // Fixed header
        n_bytes += FixedHeader {
            packet_type,
            remaining_size,
        }
        .encode(writer)?;

        writer.write_all(&content)?;
        Ok(n_bytes)
    }
}

impl Decode for Connect {
    fn decode<R: Read>(reader: &mut R) -> SageResult<Self> {
        let fixed_header = FixedHeader::decode(reader)?;

        if !matches!(fixed_header.packet_type, ControlPacketType::CONNECT) {
            return Err(Error::MalformedPacket);
        }

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

        let properties = Decode::decode(reader)?;
        // let properties = Default::default();

        Ok(Connect {
            flags,
            keep_alive,
            properties,
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
    const CONNECT_ENCODED: [u8; 19] = [
        0x10, 0xFF, 0x7F, // Change the size
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

        let properties = vec![Property::SessionExpiryInterval(10)];

        Connect {
            flags,
            keep_alive,
            properties,
        }
    }

    #[test]
    fn encode_control_connect() {
        let connect = connect_decoded();
        let mut encoded = Vec::new();

        let n_bytes = connect.encode(&mut encoded).unwrap();
        assert_eq!(encoded, CONNECT_ENCODED);
        assert_eq!(n_bytes, 19);
    }

    #[test]
    fn decode_control_connect() {
        let mut test_stream = Cursor::new(CONNECT_ENCODED);
        let connect = Connect::decode(&mut test_stream).unwrap();
        assert_eq!(connect, connect_decoded());
    }
}
