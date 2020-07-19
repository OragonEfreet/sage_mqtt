//! A set of default values for MQTT packets

use crate::QoS;

/// Default maximum qos
pub const DEFAULT_MAXIMUM_QOS: QoS = QoS::ExactlyOnce;

/// Default payload format indicator
pub const DEFAULT_PAYLOAD_FORMAT_INDICATOR: bool = false;

/// Default receive maximum
pub const DEFAULT_RECEIVE_MAXIMUM: u16 = 65_535;

/// Default request problem information
pub const DEFAULT_REQUEST_PROBLEM_INFORMATION: bool = true;

/// Default request response information
pub const DEFAULT_REQUEST_RESPONSE_INFORMATION: bool = false;

/// Default retain available
pub const DEFAULT_RETAIN_AVAILABLE: bool = true;

/// Default session expiry interval
pub const DEFAULT_SESSION_EXPIRY_INTERVAL: Option<u32> = None;

/// Default shared subscription available
pub const DEFAULT_SHARED_SUBSCRIPTION_AVAILABLE: bool = true;

/// Default topic alias maximum
pub const DEFAULT_TOPIC_ALIAS_MAXIMUM: u16 = 0;

/// Default wilcard subscription available
pub const DEFAULT_WILCARD_SUBSCRIPTION_AVAILABLE: bool = true;

/// Default will delay interval
pub const DEFAULT_WILL_DELAY_INTERVAL: u32 = 0;

/// Default subscription identifier available
pub const DEFAULT_SUBSCRIPTION_IDENTIFIER_AVAILABLE: bool = true;

/// Default keep alive
pub const DEFAULT_KEEP_ALIVE: u16 = 600;
