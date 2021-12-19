use crate::Error as SageError;
use std::convert::TryFrom;

/// A topic name a broker or client publishes to
pub type TopicName = String;

/// A topic filter a topic name matches against.
/// Clients subscribe to topic filters.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TopicFilter {
    spec: String,
}

// #[derive(Debug, Eq, PartialEq, Clone)]
// enum FilterLevel {
//     Empty,
//     Any,
//     MultipleAny,
//     Name(String),
// }

impl TryFrom<&str> for TopicFilter {
    type Error = SageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(TopicFilter { spec: s.into() })
    }
}

impl ToString for TopicFilter {
    fn to_string(&self) -> String {
        self.spec.clone()
    }
}
