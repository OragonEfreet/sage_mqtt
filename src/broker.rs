use crate::ControlPacket;
use std::io::Read;

#[derive(Default)]
pub struct Broker;

impl Broker {
    pub fn process<T: Read>(&mut self, reader: &mut T) {
        let packet = ControlPacket::decode(reader);
        println!("{:?}", packet);
    }
}
