use std::io::Read;
use crate::{Decode, ControlPacket};

#[derive(Default)]
pub struct Broker;

impl Broker {
    pub fn process<T: Read>(&mut self, reader: &mut T) {
        let packet = ControlPacket::decode(reader);
        println!("{:?}", packet);
    }
}
