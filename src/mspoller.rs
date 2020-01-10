//! mspoller: 多套接字 poller
//! 同时接收 PULL 和 SUB 套接字，并公平对待

fn main() {
    ms_poller();
}

#[cfg(feature = "feat-libzmq")]
fn ms_poller() {
    use libzmq::{prelude::*, *, poll::{PollId, Events, Poller, READABLE}};
    use std::{convert::TryInto};

    let addr: TcpAddr = "0.0.0.0:5555".try_into().unwrap();
    let pull = GatherBuilder::new().bind(addr).build().unwrap();

    let addr: TcpAddr = "0.0.0.0:5556".try_into().unwrap();
    let filter = "10001";
    let filter_group: Group = filter.try_into().unwrap();
    let sub = DishBuilder::new()
        .connect(addr)
        .join(&filter_group)
        .build()
        .unwrap();

    let mut poller = Poller::new();
    poller.add(&pull, PollId(0), READABLE).unwrap();
    poller.add(&sub, PollId(1), READABLE).unwrap();
    let mut events = Events::new();

    loop {
        poller.poll(&mut events, Period::Infinite).unwrap();
        for event in &events {
            assert!(event.is_readable());
            match event.id() {
                PollId(0) => {
                    let _pull_msg = pull.recv_msg().unwrap();
                }
                PollId(1) => {
                    let _sub_msg = sub.recv_msg().unwrap();
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(feature = "feat-nng")]
fn ms_poller() {
    use nng::{Protocol, Context, Aio, AioResult, Socket, options::{Options, protocol::pubsub::Subscribe}};

    let pull = Socket::new(Protocol::Pull0).unwrap();
    let pull_ctx = Context::new(&pull).expect("1");
    let pull_ctx_clone = pull_ctx.clone();
    let pull_aio = Aio::new(move |aio, res| pull_callback(aio, &pull_ctx_clone, res)).unwrap();
    pull.listen("tcp://0.0.0.0:5555").unwrap();

    fn pull_callback(aio: Aio, ctx: &Context, res: AioResult) {
        match res {
            AioResult::Send(Ok(_)) => ctx.recv(&aio).unwrap(),
            AioResult::Recv(Ok(_pull_msg)) => {
            }
            AioResult::Sleep(Ok(_)) => {
            }
            AioResult::Send(Err((_, e))) | AioResult::Recv(Err(e)) | AioResult::Sleep(Err(e)) => {
                panic!("Error: {}", e)
            }
        }
    }

    /////////////

    let sub = Socket::new(Protocol::Sub0).unwrap();

    let sub_ctx = Context::new(&sub).unwrap();
    let sub_ctx_clone = sub_ctx.clone();
    let sub_aio = Aio::new(move |aio, res| sub_callback(aio, &sub_ctx_clone, res)).unwrap();

    sub.dial("tcp://0.0.0.0:5556").unwrap();
    let filter = "10001";
    let all_topics = filter.as_bytes().to_vec();
    sub.set_opt::<Subscribe>(all_topics).unwrap();

    fn sub_callback(aio: Aio, ctx: &Context, res: AioResult) {
        match res {
            AioResult::Send(Ok(_)) => ctx.recv(&aio).unwrap(),
            AioResult::Recv(Ok(_pull_msg)) => {
            }
            AioResult::Sleep(Ok(_)) => {
            }
            AioResult::Send(Err((_, e))) | AioResult::Recv(Err(e)) | AioResult::Sleep(Err(e)) => {
                panic!("Error: {}", e)
            }
        }
    }
    //////////
    pull_ctx.recv(&pull_aio).unwrap();
    sub_ctx.recv(&sub_aio).unwrap();

    std::thread::park();
}