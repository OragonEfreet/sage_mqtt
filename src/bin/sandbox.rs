use sage_mqtt::*;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let s = "Test";
    s.encode(&mut handle).unwrap();
}
