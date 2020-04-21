use crate::{Decode, Encode, Result as SageResult};

use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct Publish {}

impl Default for Publish {
    fn default() -> Self {
        Publish {}
    }
}

impl Decode for Publish {
    fn decode<R: Read>(_: &mut R) -> SageResult<Self> {
        unimplemented!();
    }
}

impl Encode for Publish {
    fn encode<W: Write>(self, _: &mut W) -> SageResult<usize> {
        unimplemented!();
    }
}
