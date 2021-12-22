use std::fmt;

mod filter;
pub use filter::TopicFilter;

const LEVEL_SEPARATOR: char = '/';

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum TopicLevel {
    Empty,
    Name(String),
}
