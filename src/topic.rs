use crate::Error as SageError;
use std::convert::{AsRef, TryFrom};

/// A topic name a broker or client publishes to
pub type TopicName = String;

/// A topic filter a topic name matches against.
/// Clients subscribe to topic filters.
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct TopicFilter {
    spec: String,
}

impl TryFrom<&str> for TopicFilter {
    type Error = SageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from(String::from(s))
    }
}

impl TryFrom<String> for TopicFilter {
    type Error = SageError;

    fn try_from(spec: String) -> Result<Self, Self::Error> {
        Ok(TopicFilter { spec })
    }
}

impl AsRef<str> for TopicFilter {
    fn as_ref(&self) -> &str {
        &self.spec
    }
}
