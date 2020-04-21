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
mod connect;
mod control_packet;
mod control_packet_type;
mod decode;
mod defaults;
mod encode;
mod error;
mod fixed_header;
mod property;
mod property_id;
mod quality_of_service;
mod reason_code;
mod types;

pub use broker::Broker;
pub use connect::Connect;
pub use control_packet::ControlPacket;
use control_packet_type::ControlPacketType;
pub use decode::Decode;
use defaults::{
    DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_RECEIVE_MAXIMUM, DEFAULT_REQUEST_PROBLEM_INFORMATION,
    DEFAULT_REQUEST_RESPONSE_INFORMATION, DEFAULT_SESSION_EXPIRY_INTERVAL,
    DEFAULT_TOPIC_ALIAS_MAXIMUM, DEFAULT_WILL_DELAY_INTERVAL,
};
pub use encode::Encode;
pub use error::{Error, Result};
use fixed_header::FixedHeader;
use property::{PropertiesDecoder, Property};
use property_id::PropertyId;
pub use quality_of_service::QoS;
pub use reason_code::ReasonCode;
pub use types::{
    BinaryData, Bits, Byte, FourByteInteger, TwoByteInteger, UTF8String, VariableByteInteger,
};
