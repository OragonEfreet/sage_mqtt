use sage_mqtt::VariableByteInteger;
// use std::io;

fn main() {
    println!(">>> {:X?}", VariableByteInteger::from(1984_u16));
}
