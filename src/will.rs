use crate::{
    defaults::{DEFAULT_PAYLOAD_FORMAT_INDICATOR, DEFAULT_WILL_DELAY_INTERVAL},
    QoS, Topic,
};

/// Due to the unstable nature of a connexion, the client can loose its
/// connection to the server. This ungraceful disconnect can be notified
/// to every other clients by specifying a Last Will message that is given
/// upon connection.
/// When a client ungracefully disconnect from a server (when the keep alive
/// is reached), the server will publish the Last Will message to anyone
/// subscribed to its topic.
#[derive(Debug, PartialEq, Clone)]
pub struct Will {
    /// The quality of service for the will message.
    pub qos: QoS,

    /// If the message is to be retained. A retain message is kept
    /// in memory by a broker (one per topic) to sent to future subscriptions.
    pub retain: bool,

    /// Delay in seconds the broker will wait after a deconnection before
    /// publishing the will message. The will message can also be published
    /// at session expires if it happens first.
    pub delay_interval: u32,

    /// If true, the will message will be a valid UTF-8 encoded string. If not
    /// the will message can be anything, even a unicorn.
    pub payload_format_indicator: bool,

    /// Corresponds to the expiry interval of the `Publish` message sent.
    pub message_expiry_interval: Option<u32>,

    /// Describes the type of content of the payload. Is generally a MIME
    /// descriptor.
    pub content_type: String,

    /// Optional topic used as response if the Will message is a request.
    pub response_topic: Option<Topic>,

    /// Optional correlation optionaly used if the Will message is a request.
    pub correlation_data: Option<Vec<u8>>,

    /// General purpose properties
    pub user_properties: Vec<(String, String)>,

    /// The Last Will Topic. Cannot be empty.
    pub topic: Topic,

    /// The last will payload.
    pub message: Vec<u8>,
}

impl Will {
    /// Builds a default Will with specified topic and message
    pub fn with_message(topic: Topic, message: &str) -> Self {
        Will {
            qos: QoS::AtMostOnce,
            retain: false,
            delay_interval: DEFAULT_WILL_DELAY_INTERVAL,
            payload_format_indicator: DEFAULT_PAYLOAD_FORMAT_INDICATOR,
            message_expiry_interval: None,
            content_type: Default::default(),
            response_topic: None,
            correlation_data: None,
            user_properties: Default::default(),
            topic,
            message: message.as_bytes().to_vec(),
        }
    }
}
