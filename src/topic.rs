use crate::Error as SageError;
use std::{convert::TryFrom, fmt};

const LEVEL_SEPARATOR: char = '/';

/// A topic name a broker or client publishes to
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TopicName(String);

impl TryFrom<&str> for TopicName {
    type Error = SageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(TopicName(s.into()))
    }
}

impl fmt::Display for TopicName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A topic filter a topic name matches against.
/// Clients subscribe to topic filters.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TopicFilter {
    levels: Vec<FilterSegment>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TopicLevel {
    Empty,
    Name(String),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum FilterSegment {
    Any,
    MultipleAny,
    Level(TopicLevel),
}

impl TryFrom<&str> for TopicFilter {
    type Error = SageError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(TopicFilter {
            levels: s
                .split(LEVEL_SEPARATOR)
                .into_iter()
                .map(|l| {
                    if l.len() == 0 {
                        FilterSegment::Level(TopicLevel::Empty)
                    } else {
                        match l {
                            "+" => FilterSegment::Any,
                            "#" => FilterSegment::MultipleAny,
                            _ => FilterSegment::Level(TopicLevel::Name(l.into())),
                        }
                    }
                })
                .collect(),
        })
    }
}

impl fmt::Display for TopicFilter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.levels
                .iter()
                .map(|l| match l {
                    FilterSegment::Any => "+",
                    FilterSegment::MultipleAny => "#",
                    FilterSegment::Level(TopicLevel::Empty) => "",
                    FilterSegment::Level(TopicLevel::Name(s)) => s.as_ref(),
                })
                .collect::<Vec<&str>>()
                .join("/")
        )
    }
}
