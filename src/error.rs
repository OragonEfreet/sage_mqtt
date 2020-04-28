use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IOError,
    result::Result as StdResult,
};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Standard Rust IO Error
    Io(IOError),

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

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::Io(err)
    }
}

impl Error {
    pub(crate) fn reason_code(&self) -> Option<u8> {
        match *self {
            Error::MalformedPacket => Some(0x81),
            Error::ProtocolError => Some(0x82),
            Error::ReceiveMaximumExceeded => Some(0x93),
            Error::PacketTooLarge => Some(0x95),
            Error::RetainNotSupported => Some(0x9A),
            Error::QoSNotSupported => Some(0x9B),
            Error::SharedSubscriptionNotSupported => Some(0x9E),
            Error::SubscriptionIdentifiersNotSupported => Some(0xA1),
            Error::WildcardSubscriptionsNotSupported => Some(0xA2),
            _ => None,
        }
    }
}
