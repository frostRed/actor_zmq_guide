//! hwclient: Hello World 客户端
//! 一个 REQ 套接字连向 tcp::*:5555
//! 发送 Hello

fn main() {
    hw_client()
}

#[cfg(feature = "feat-libzmq")]
fn hw_client() {
    use libzmq::{prelude::*, *};
    use std::convert::TryInto;

    let addr: TcpAddr = "0.0.0.0:5555".try_into().unwrap();
    let client = ClientBuilder::new().connect(addr).build().unwrap();

    client.send("Hello").unwrap();

    let msg = client.recv_msg().unwrap();
    println!("Received: {:?}", msg.to_str().unwrap());
}

#[cfg(feature = "feat-nng")]
fn hw_client() {
    use nng::{Message, Protocol, Socket, options::{Options, protocol::pubsub::Subscribe}};
    use std::io::Write;

    let pull = Socket::new(Protocol::Pull0).unwrap();
    pull.dial("tcp://0.0.0.0:5555").unwrap();

    let sub = Socket::new(Protocol::Sub0).unwrap();
    sub.dial("tcp://0.0.0.0:5556").unwrap();
    let filter = "10001";
    let all_topics = filter.as_bytes().to_vec();
    sub.set_opt::<Subscribe>(all_topics).unwrap();
}
