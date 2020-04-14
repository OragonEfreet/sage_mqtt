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

mod control_packet;
mod control_packet_type;
mod decode;
mod encode;
mod error;
mod property;
mod property_id;
mod types;

pub use control_packet::ControlPacket;
pub use control_packet_type::ControlPacketType;
pub use decode::Decode;
pub use encode::Encode;
pub use error::{Error, Result};
use property::Property;
use property_id::PropertyId;
pub use types::{
    BinaryData, Bits, FourByteInteger, TwoByteInteger, UTF8String, VariableByteInteger,
};
