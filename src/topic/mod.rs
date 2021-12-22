use std::fmt;

mod filter;
pub use filter::TopicFilter;

const LEVEL_SEPARATOR: char = '/';

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum TopicLevel {
    Empty,
    Name(String),
}

/// A topic name a broker or client publishes to
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub struct TopicName {
    spec: Vec<TopicLevel>,
}

impl Default for TopicName {
    fn default() -> Self {
        TopicName {
            spec: vec![TopicLevel::Empty],
        }
    }
}

impl From<String> for TopicName {
    fn from(s: String) -> Self {
        Self::from(s.as_ref())
    }
}

impl From<&str> for TopicName {
    fn from(s: &str) -> Self {
        TopicName {
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
}

impl fmt::Display for TopicName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}",
            self.spec
                .iter()
                .map(|l| match l {
                    TopicLevel::Empty => "",
                    TopicLevel::Name(s) => s.as_ref(),
                })
                .collect::<Vec<&str>>()
                .join("/")
        )
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
                        assert_eq!(TopicName::from(String::from(input)), TopicName {spec});
                    }

                    #[test]
                    fn from_str_ref() {
                        let (input, spec) = $value;
                        assert_eq!(TopicName::from(input), TopicName {spec});
                    }

                    #[test]
                    fn to_string() {
                        let (input, spec) = $value;
                        assert_eq!(TopicName {spec}.to_string(), input);
                    }
                }
            )*
        }
    }

    topic_name_data! {
        default:          (String::default(), vec![TopicLevel::Empty], ),
        space:            (" ",               vec![TopicLevel::Name(String::from(" "))], ),
        empty_1:          ("",                vec![TopicLevel::Empty ; 1], ),
        empty_2:          ("/",               vec![TopicLevel::Empty ; 2], ),
        empty_3:          ("//",              vec![TopicLevel::Empty ; 3], ),
        single:           ("jaden",           vec![TopicLevel::Name(String::from("jaden"))], ),
        single_head:      ("/jaden",          vec![TopicLevel::Empty, TopicLevel::Name(String::from("jaden"))], ),
        single_tail:      ("jaden/",          vec![TopicLevel::Name(String::from("jaden")), TopicLevel::Empty], ),
        single_wrapped:   ("/jaden/",         vec![TopicLevel::Empty, TopicLevel::Name(String::from("jaden")), TopicLevel::Empty], ),
        multiple:         ("jaden/jarod",     vec![TopicLevel::Name(String::from("jaden")), TopicLevel::Name(String::from("jarod"))], ),
        multiple_head:    ("/jaden/jarod",    vec![TopicLevel::Empty, TopicLevel::Name(String::from("jaden")), TopicLevel::Name(String::from("jarod"))], ),
        multiple_tail:    ("jaden/jarod/",    vec![TopicLevel::Name(String::from("jaden")), TopicLevel::Name(String::from("jarod")), TopicLevel::Empty], ),
        multiple_wrapped: ("/jaden/jarod/",   vec![TopicLevel::Empty, TopicLevel::Name(String::from("jaden")), TopicLevel::Name(String::from("jarod")), TopicLevel::Empty], ),
        wildcard_plus:    ("+",               vec![TopicLevel::Name(String::from("+"))], ),
        wildcard_pound:   ("#",               vec![TopicLevel::Name(String::from("#"))], ),
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
