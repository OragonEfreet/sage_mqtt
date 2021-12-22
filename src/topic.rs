use std::fmt;

const LEVEL_SEPARATOR: char = '/';

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum TopicLevel {
    Empty,
    Name(String),
    Share(String),
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
                    TopicLevel::Share(_) => "Shareuh",
                    TopicLevel::Any => "+",
                    TopicLevel::MultipleAny => "#",
                })
                .collect::<Vec<&str>>()
                .join("/")
        )
    }
}

impl From<String> for Topic {
    fn from(s: String) -> Self {
        Topic::from(s.as_ref())
    }
}

impl From<&str> for Topic {
    /// Builds a new topic has a name
    fn from(s: &str) -> Self {
        let (mut shared, topic) = {
            let stripped = s.strip_prefix("$share/");
            (stripped.is_some(), stripped.unwrap_or(s))
        };

        let spec: Vec<TopicLevel> = topic
            .split(LEVEL_SEPARATOR)
            .into_iter()
            .map(|l| {
                if shared {
                    shared = false;
                    Some(TopicLevel::Share(l.into()))
                } else {
                    if l.len() == 0 {
                        Some(TopicLevel::Empty)
                    } else {
                        match l {
                            "+" => Some(TopicLevel::Any),
                            "#" => Some(TopicLevel::MultipleAny),
                            _ => Some(TopicLevel::Name(l.into())),
                        }
                    }
                }
            })
            .filter_map(|e| e)
            .collect();
        // TODO maybe use fold instead

        Topic { spec }
    }
}

impl Topic {
    /// Returns the name of the share if any
    pub fn share(&self) -> Option<String> {
        None
    }

    /// Checks whether the topic contains any wildcard
    pub fn has_wildcards(&self) -> bool {
        self.spec
            .iter()
            .any(|l| matches!(l, TopicLevel::Any | TopicLevel::MultipleAny))
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    macro_rules! test_data {
        ($($name:ident: $value:expr,)*) => {
            $(
                mod $name {
                    use super::*;
                    #[test]
                    fn from_string() {
                        let (input, spec) = $value;
                        assert_eq!(Topic::from(input), Topic {spec});
                    }

                    #[test]
                    fn from_str_ref() {
                        let (input, spec) = $value;
                        assert_eq!(Topic::from(input), Topic {spec});
                    }

                }
            )*
        }
    }

    use super::TopicLevel::*;

    test_data! {
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
        wildcard_plus:    ("+",               vec![Any], ),
        wildcard_pound:   ("#",               vec![MultipleAny], ),
        // Shared
        share_space:            ("$share/ ",               vec![Share(" ".into())], ),
        share_empty_1:          ("$share/",                vec![Share("".into())], ),
        share_empty_2:          ("$share//",               vec![Share("".into()), Empty], ),
        share_empty_3:          ("$share///",              vec![Share("".into()), Empty, Empty], ),
        share_single:           ("$share/jaden",           vec![Share("jaden".into())], ),
        share_single_head:      ("$share//jaden",          vec![Share("".into()), Name("jaden".into())], ),
        share_single_tail:      ("$share/jaden/",          vec![Share("jaden".into()), Empty], ),
        share_single_wrapped:   ("$share//jaden/",         vec![Share("".into()), Name("jaden".into()), Empty], ),
        share_multiple:         ("$share/jaden/jarod",     vec![Share("jaden".into()), Name("jarod".into())], ),
        share_multiple_head:    ("$share//jaden/jarod",    vec![Share("".into()), Name("jaden".into()), Name("jarod".into())], ),
        share_multiple_tail:    ("$share/jaden/jarod/",    vec![Share("jaden".into()), Name("jarod".into()), Empty], ),
        share_multiple_wrapped: ("$share//jaden/jarod/",   vec![Share("".into()), Name("jaden".into()), Name("jarod".into()), Empty], ),
        share_wildcard_plus_1:  ("$share/+",               vec![Share("+".into())], ),
        share_wildcard_pound_1: ("$share/#",               vec![Share("#".into())], ),
        share_wildcard_plus_2:  ("$share/+/+",             vec![Share("+".into()), Any], ),
        share_wildcard_pound_2: ("$share/#/#",             vec![Share("#".into()), MultipleAny], ),
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
