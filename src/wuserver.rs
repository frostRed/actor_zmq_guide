//!  天气更新服务器
//!  PUB 套接字绑向 tcp://0.0.0.0:5556
//!  发布随机的天气更新

use rand::Rng;

fn main() {
    wu_server();
}

#[cfg(feature = "feat-libzmq")]
fn wu_server() {
    use libzmq::{prelude::*, *};
    use std::convert::TryInto;

    let addr: TcpAddr = "0.0.0.0:5556".try_into().unwrap();

    let server = RadioBuilder::new().bind(addr).build().unwrap();

    let mut rng = rand::thread_rng();

    loop {
        let zipcode: i64 = rng.gen_range(0, 100000);
        let temperature: i64 = rng.gen_range(0, 215) - 80;
        let relhumidity: i64 = rng.gen_range(0, 50) - 10;
        let msg = format!("{:05} {} {}", zipcode, temperature, relhumidity);
        let group: Group = format!("{}", zipcode).try_into().unwrap();

        println!("Weather update: {}", msg);
        server.transmit(msg, group).unwrap();
    }
}

#[cfg(feature = "feat-nng")]
fn wu_server() {
    use nng::{Protocol, Socket, Message};
    use std::io::Write;

    let s= Socket::new(Protocol::Pub0).unwrap();
    s.listen("tcp://0.0.0.0:5556").unwrap();

    let mut rng = rand::thread_rng();
    loop {
        let zipcode: i64 = rng.gen_range(0, 100000);
        let temperature: i64 = rng.gen_range(0, 215) - 80;
        let relhumidity: i64 = rng.gen_range(0, 50) - 10;

        let mut msg = Message::new().unwrap();
        msg.write_fmt(format_args!("{:05} {} {}", zipcode, temperature, relhumidity)).unwrap();

        s.send(msg).unwrap();
    }
}