use std::fmt;

const LEVEL_SEPARATOR: char = '/';

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum TopicLevel {
    Empty,
    Name(String),
    Any,
    MultipleAny,
}

/// A topic name a broker or client publishes to
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub struct Topic {
    spec: Vec<TopicLevel>,
}

impl Default for Topic {
    fn default() -> Self {
        Topic {
            spec: vec![TopicLevel::Empty],
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.spec
                .iter()
                .map(|l| match l {
                    TopicLevel::Empty => "",
                    TopicLevel::Name(s) => s.as_ref(),
                    TopicLevel::Any => "+",
                    TopicLevel::MultipleAny => "#",
                })
                .collect::<Vec<&str>>()
                .join("/")
        )
    }
}

impl Topic {
    /// Builds a new topic has a name
    pub fn name(s: &str) -> Self {
        Topic {
            spec: s
                .split(LEVEL_SEPARATOR)
                .into_iter()
                .map(|l| {
                    if l.len() == 0 {
                        TopicLevel::Empty
                    } else {
                        TopicLevel::Name(l.into())
                    }
                })
                .collect(),
        }
    }

    /// Builds a new topic has a name
    pub fn filter(s: &str) -> Self {
        Topic {
            spec: s
                .split(LEVEL_SEPARATOR)
                .into_iter()
                .map(|l| {
                    if l.len() == 0 {
                        TopicLevel::Empty
                    } else {
                        match l {
                            "+" => TopicLevel::Any,
                            "#" => TopicLevel::MultipleAny,
                            _ => TopicLevel::Name(l.into()),
                        }
                    }
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    macro_rules! topic_name_data {
        ($($name:ident: $value:expr,)*) => {
            $(
                mod $name {
                    use super::*;
                    #[test]
                    fn from_string() {
                        let (input, spec) = $value;
                        assert_eq!(Topic::name(input), Topic {spec});
                    }

                    #[test]
                    fn from_str_ref() {
                        let (input, spec) = $value;
                        assert_eq!(Topic::name(input), Topic {spec});
                    }

                }
            )*
        }
    }

    use super::TopicLevel::*;

    topic_name_data! {
        space:            (" ",               vec![Name(" ".into())], ),
        empty_1:          ("",                vec![Empty ; 1], ),
        empty_2:          ("/",               vec![Empty ; 2], ),
        empty_3:          ("//",              vec![Empty ; 3], ),
        single:           ("jaden",           vec![Name("jaden".into())], ),
        single_head:      ("/jaden",          vec![Empty, Name("jaden".into())], ),
        single_tail:      ("jaden/",          vec![Name("jaden".into()), Empty], ),
        single_wrapped:   ("/jaden/",         vec![Empty, Name("jaden".into()), Empty], ),
        multiple:         ("jaden/jarod",     vec![Name("jaden".into()), Name("jarod".into())], ),
        multiple_head:    ("/jaden/jarod",    vec![Empty, Name("jaden".into()), Name("jarod".into())], ),
        multiple_tail:    ("jaden/jarod/",    vec![Name("jaden".into()), Name("jarod".into()), Empty], ),
        multiple_wrapped: ("/jaden/jarod/",   vec![Empty, Name("jaden".into()), Name("jarod".into()), Empty], ),
        wildcard_plus:    ("+",               vec![Name("+".into())], ),
        wildcard_pound:   ("#",               vec![Name("#".into())], ),
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(
            Topic::default(),
            Topic {
                spec: vec![TopicLevel::Empty],
            },
        );
    }
}
