use async_std::io::Error as AsyncIOError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

/// Standard Result type for Sage MQTT
pub type Result<T> = StdResult<T, Error>;

/// The error type for Sage MQTT operations
#[derive(Debug)]
pub enum Error {
    /// Standard Rust IO Error
    Io(AsyncIOError),

    /// 0x81: Malformed Packet
    MalformedPacket,

    /// 0x82: Protocol Error
    ProtocolError,

    /// 0x93: Receive Maximum exceeded
    ReceiveMaximumExceeded,

    /// 0x95: Packet too large
    PacketTooLarge,

    /// 0x9A: Retain not supported
    RetainNotSupported,

    /// 0x9B: QoS not supported
    QoSNotSupported,

    /// 0x9E: Shared Subscriptions not supported
    SharedSubscriptionNotSupported,

    /// 0xA1: Subscription Identifiers not supported
    SubscriptionIdentifiersNotSupported,

    /// 0xA2: Wildcard Subscriptions not supported
    WildcardSubscriptionsNotSupported,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::Io(ref e) => e.fmt(f),
            Error::MalformedPacket => write!(f, "Malformed Packet"),
            Error::ProtocolError => write!(f, "Protocol Error"),
            Error::ReceiveMaximumExceeded => write!(f, "Receive Maximum exceeded"),
            Error::PacketTooLarge => write!(f, "Packet too large"),
            Error::RetainNotSupported => write!(f, "Retain not supported"),
            Error::QoSNotSupported => write!(f, "QoS not supported"),
            Error::SharedSubscriptionNotSupported => {
                write!(f, "Shared Subscriptions not supported")
            }
            Error::SubscriptionIdentifiersNotSupported => {
                write!(f, "Subscription Identifiers not supported")
            }
            Error::WildcardSubscriptionsNotSupported => {
                write!(f, "Wildcard Subscriptions not supported")
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<AsyncIOError> for Error {
    fn from(err: AsyncIOError) -> Self {
        Error::Io(err)
    }
}
