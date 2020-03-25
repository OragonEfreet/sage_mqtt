//! `sage_mqtt` is a set of traits and
#[allow(unused_macros)]
macro_rules! assert_matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        assert!(matches!($expression, $( $pattern )|+ $( if $guard )?))
    }
}

mod encode;
mod error;
mod types;

pub use encode::Encode;
pub use error::{Error, Result};
pub use types::{
    BinaryData, Bits, FourByteInteger, TwoByteInteger, UTF8String, VariableByteInteger,
};
