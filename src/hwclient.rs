//! hwclient: Hello World 客户端
//! 一个 REQ 套接字连向 tcp::*:5555
//! 发送 Hello

fn main() {
    hw_client()
}

#[cfg(feature = "feat-libzmq")]
fn hw_client() {
    use std::convert::TryInto;
    use libzmq::{prelude::*, *};

    let addr: TcpAddr = "0.0.0.0:5555".try_into().unwrap();
    let client = ClientBuilder::new().connect(addr).build().unwrap();

    client.send("Hello").unwrap();

    let msg = client.recv_msg().unwrap();
    println!("Received: {:?}", msg.to_str().unwrap());
}

#[cfg(feature = "feat-nng")]
fn hw_client() {
    use nng::{Message, Protocol, Socket};
    use std::io::Write;


    let s = Socket::new(Protocol::Req0).unwrap();
    s.dial("tcp://0.0.0.0:5555").unwrap();

    let mut req = Message::new().unwrap();
    req.write("Hello".as_bytes()).unwrap();
    s.send(req).unwrap();

    let msg = s.recv().unwrap();
    println!(
        "Received: {:?}",
        String::from_utf8(msg.as_slice().to_vec()).unwrap()
    );
}