//  天气更新客户端
//  SUB 套接字连向 tcp://0.0.0.0:5556
//  收集天气更新，找到某个地区的平均气温

fn main() {
    wu_client();
}

#[cfg(feature = "feat-libzmq")]
fn wu_client() {
    use libzmq::{prelude::*, *};
    use std::convert::TryInto;

    let addr: TcpAddr = "0.0.0.0:5556".try_into().unwrap();
    let filter = "10001";
    let filter_group: Group = filter.try_into().unwrap();
    let client = DishBuilder::new()
        .connect(addr)
        .join(&filter_group)
        .build()
        .unwrap();

    let mut total_temp = 0i64;
    let mut update_nbr = 0;
    while update_nbr < 100 {
        let msg = client.recv_msg().unwrap();
        let msg = msg.to_str().unwrap();
        println!("Received weather update: {}", msg);
        let msg = msg.split(' ').collect::<Vec<&str>>();

        total_temp += msg[1].parse::<i64>().unwrap();
        update_nbr += 1;
    }
    println!(
        "Average temperature for zipcode {} was {}F",
        filter,
        total_temp / update_nbr
    );
}
#[cfg(feature = "feat-nng")]
fn wu_client() {
    // nng 的 PUB-SUB，其中 PUB 必须先启动。
    use nng::{Protocol, Socket, options::{Options, protocol::pubsub::Subscribe}};

    let client = Socket::new(Protocol::Sub0).expect("1");
    client.dial("tcp://0.0.0.0:5556").expect("2");


    let filter = "10001";
    let all_topics = filter.as_bytes().to_vec();
    client.set_opt::<Subscribe>(all_topics).expect("3");

    let mut total_temp = 0i64;
    let mut update_nbr = 0;
    while update_nbr < 100 {
        let msg = client.recv().expect("4");
        let msg = String::from_utf8(msg.as_slice().to_vec()).expect("5");

        println!("Received weather update: {}", msg);
        let msg = msg.split(' ').collect::<Vec<&str>>();

        total_temp += msg[1].parse::<i64>().expect("6");
        update_nbr += 1;
    }
    println!(
        "Average temperature for zipcode {} was {}F",
        filter,
        total_temp / update_nbr
    );
}
