//! hwserver: Hello World 服务器
//! 绑定 REP 套接字在 tcp://*:5555
//! 期待客户端发来“Hello”，然后响应“World”


fn main() {
    hw_server();
}

#[cfg(feature = "feat-libzmq")]
fn hw_server() {
    use std::{convert::TryInto, thread::sleep, time::Duration};
    use libzmq::{prelude::*, *};

    let addr: TcpAddr = "0.0.0.0:5555".try_into().unwrap();

    let server = ServerBuilder::new().bind(addr).build().unwrap();

    loop {
        let msg = server.recv_msg().unwrap();
        println!("Received Hello: {:?}", msg.to_str().unwrap());
        let id = msg.routing_id().unwrap();

        sleep(Duration::from_secs(1));

        server.route("World", id).unwrap();
    }
}

#[cfg(feature = "feat-nng")]
fn hw_server() {
    use std::{io::Write, thread::sleep, time::Duration};
    use nng::{Protocol, Socket};

    let s = Socket::new(Protocol::Rep0).unwrap();
    s.listen("tcp://0.0.0.0:5555").unwrap();

    loop {
        let mut msg = s.recv().unwrap();
        println!(
            "Received Hello: {:?}",
            String::from_utf8(msg.as_slice().to_vec()).unwrap()
        );

        sleep(Duration::from_secs(1));

        msg.clear();
        msg.write("World".as_bytes()).unwrap();
        s.send(msg).unwrap();
    }
}
