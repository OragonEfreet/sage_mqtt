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

#[cfg(test)]
mod unit {
    use super::*;

    macro_rules! topic_filter_data {
        ($($name:ident: $value:expr,)*) => {
            $(
                mod $name {
                    use super::*;
                    #[test]
                    fn from_string() {
                        let (input, spec) = $value;
                        assert_eq!(TopicFilter::from(String::from(input)), TopicFilter {share: None, spec});
                    }

                    #[test]
                    fn from_str_ref() {
                        let (input, spec) = $value;
                        assert_eq!(TopicFilter::from(input), TopicFilter {share: None, spec});
                    }

                    #[test]
                    fn to_string() {
                        let (input, spec) = $value;
                        assert_eq!(TopicFilter {share: None, spec}.to_string(), input);
                    }
                }
            )*
        }
    }

    topic_filter_data! {
        default:          (String::default(), vec![FilterSegment::Level(TopicLevel::Empty)], ),
        space:            (" ",               vec![FilterSegment::Level(TopicLevel::Name(String::from(" ")))], ),
        empty_1:          ("",                vec![FilterSegment::Level(TopicLevel::Empty) ; 1], ),
        empty_2:          ("/",               vec![FilterSegment::Level(TopicLevel::Empty) ; 2], ),
        empty_3:          ("//",              vec![FilterSegment::Level(TopicLevel::Empty) ; 3], ),
        single:           ("jaden",           vec![FilterSegment::Level(TopicLevel::Name(String::from("jaden")))], ),
        single_head:      ("/jaden",          vec![FilterSegment::Level(TopicLevel::Empty), FilterSegment::Level(TopicLevel::Name(String::from("jaden")))], ),
        single_tail:      ("jaden/",          vec![FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Empty)], ),
        single_wrapped:   ("/jaden/",         vec![FilterSegment::Level(TopicLevel::Empty), FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Empty)], ),
        multiple:         ("jaden/jarod",     vec![FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Name(String::from("jarod")))], ),
        multiple_head:    ("/jaden/jarod",    vec![FilterSegment::Level(TopicLevel::Empty), FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Name(String::from("jarod")))], ),
        multiple_tail:    ("jaden/jarod/",    vec![FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Name(String::from("jarod"))), FilterSegment::Level(TopicLevel::Empty)], ),
        multiple_wrapped: ("/jaden/jarod/",   vec![FilterSegment::Level(TopicLevel::Empty), FilterSegment::Level(TopicLevel::Name(String::from("jaden"))), FilterSegment::Level(TopicLevel::Name(String::from("jarod"))), FilterSegment::Level(TopicLevel::Empty)], ),
        wildcard_plus:    ("+",               vec![FilterSegment::Any], ),
        wildcard_pound:   ("#",               vec![FilterSegment::MultipleAny],),
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(
            TopicName::default(),
            TopicName {
                spec: vec![TopicLevel::Empty],
            },
        );
    }
}
