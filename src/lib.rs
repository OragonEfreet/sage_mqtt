//! `sage_mqtt` is a an encoding/decoding library for MQTT 5.0 protocol.
//! The library consists in pivot types, such as `UTF8String` that can be
//! written to and read from a stream as well as converted to standard Rust
//! types.

#[allow(unused_macros)]
macro_rules! assert_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        assert!(matches!($expression, $( $pattern )|+ $( if $guard )?))
    }
}

mod broker;
mod codec;
mod control_packets;
mod error;
mod reason_code;

pub use broker::Broker;
pub use codec::{
    BinaryData, Bits, Byte, Decode, Encode, FourByteInteger, QoS, TwoByteInteger, UTF8String,
    VariableByteInteger,
};
pub use control_packets::{Connect, ControlPacket};
use control_packets::{
    ControlPacketType, FixedHeader, PropertiesDecoder, Property, PropertyId,
    DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM, DEFAULT_REQUEST_PROBLEM_INFORMATION,
    DEFAULT_REQUEST_RESPONSE_INFORMATION, DEFAULT_SESSION_EXPIRY_INTERVAL,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILL_DELAY_INTERVAL,
};
pub use error::{Error, Result};
pub use reason_code::ReasonCode;
