use crate::Result as SageResult;
use std::io::Write;

/// The `Encode` trait describes how to write a type into an MQTT stream.
pub trait Encode {
    /// Encodes `this` and writes it into `write`, returning how many bytes
    /// were written.
    fn encode<W: Write>(self, writer: &mut W) -> SageResult<usize>;
}
