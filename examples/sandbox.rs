use sage_mqtt::Topic;

fn main() {
    let topic = "/pouet//haha/+/chaise/#/";
    let pouet = Topic::filter(topic);

    println!("{}", pouet);
    println!("{}", pouet);
}
