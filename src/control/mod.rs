/// This module lists all the encode/decode functions used to read the
/// fundamental types specified within MQTT5:
/// - byte and boolean values
/// - UTF8 String
/// - 2, 4 and variable byte integers
/// - Binary Data
/// - Quality of service
/// - Reason Codes
mod auth;
mod connack;
mod connect;
mod disconnect;
mod puback;
mod pubcomp;
mod publish;
mod pubrec;
mod pubrel;
mod suback;
mod subscribe;
mod unsuback;
mod unsubscribe;

/// String alias to represent a client identifier
pub type ClientID = String;

pub use auth::Auth;
pub use connack::ConnAck;
pub use connect::Connect;
pub use disconnect::Disconnect;
pub use puback::PubAck;
pub use pubcomp::PubComp;
pub use publish::Publish;
pub use pubrec::PubRec;
pub use pubrel::PubRel;
pub use suback::SubAck;
pub use subscribe::{RetainHandling, Subscribe, SubscriptionOptions};
pub use unsuback::UnSubAck;
pub use unsubscribe::UnSubscribe;

/// A ping request message
pub struct PingReq;

/// A ping response message
pub struct PingResp;
