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

mod control_packets;
mod decode;
mod encode;
mod error;
mod property;
mod property_id;
mod quality_of_service;
mod reason_code;
mod types;

pub use control_packets::Connect;
pub use decode::Decode;
pub use encode::Encode;
pub use error::{Error, Result};
pub use quality_of_service::QoS;
pub use reason_code::ReasonCode;
pub use types::{
    BinaryData, Bits, Byte, FourByteInteger, TwoByteInteger, UTF8String, VariableByteInteger,
};

use control_packets::{ControlPacketType, FixedHeader};
use property::{Properties, Property};
use property_id::PropertyId;
