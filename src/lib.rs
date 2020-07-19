//! `sage_mqtt` is a an encoding/decoding library for MQTT 5.0 protocol.
//! The library consists in pivot types, such as `UTF8String` that can be
//! written to and read from a stream as well as converted to standard Rust
//! types.
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
#![allow(clippy::large_enum_variant)]

#[allow(unused_macros)]
macro_rules! assert_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        assert!(matches!($expression, $( $pattern )|+ $( if $guard )?))
    }
}

mod authentication;
/// encode/decode MQTT fundamental types
pub mod codec;
mod control;
pub mod defaults;
mod error;
mod packet;
mod packet_type;
mod property;
mod quality_of_service;
mod reason_code;
mod will;
pub use authentication::Authentication;
pub use control::{
    Auth, ClientID, ConnAck, Connect, Disconnect, PingReq, PingResp, PubAck, PubComp, PubRec,
    PubRel, Publish, RetainHandling, SubAck, Subscribe, SubscriptionOptions, UnSubAck, UnSubscribe,
};
pub use error::{Error, Result};
pub use packet::Packet;
use packet_type::PacketType;
use property::{PropertiesDecoder, Property};
pub use quality_of_service::QoS;
pub use reason_code::ReasonCode;
pub use will::Will;
