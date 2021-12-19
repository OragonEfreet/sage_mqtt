use sage_mqtt::TopicFilter;
use std::convert::TryFrom;

fn main() {
    let topic = "/pouet//haha/+/chaise/#/";
    let pouet = TopicFilter::try_from(topic).unwrap();

    println!("{}", pouet);
    println!("{}", pouet);
}
