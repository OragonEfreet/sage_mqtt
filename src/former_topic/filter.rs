use super::*;

/// A topic filter a topic name matches against.
/// Clients subscribe to topic filters.
#[derive(Default, Hash, Debug, Eq, PartialEq, Clone)]
pub struct TopicFilter {
    spec: Vec<FilterSegment>,
    share: Option<String>,
}

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum FilterSegment {
    Any,
    MultipleAny,
    Level(TopicLevel),
}

impl From<String> for TopicFilter {
    fn from(s: String) -> Self {
        Self::from(s.as_ref())
    }
}

impl From<&str> for TopicFilter {
    fn from(s: &str) -> Self {
        TopicFilter {
            share: None,
            spec: s
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
        }
    }
}

impl fmt::Display for TopicFilter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.spec
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

impl TopicFilter {
    /// Returns true if the filter contains at least one wildcard
    pub fn has_wildcards(&self) -> bool {
        self.spec
            .iter()
            .any(|x| matches!(x, FilterSegment::Any | FilterSegment::MultipleAny))
    }

    /// Returns the sharing name if the topic is shared
    pub fn share(&self) -> &Option<String> {
        &self.share
    }
}
