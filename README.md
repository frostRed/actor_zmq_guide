# zmq_guide
Try 0MQ guide example by a variety of rust crates.

### libzmq-rs

```rust
req.send(String);
rep.route(String, RoutingId);
req.send(msg.set_routing_id())

let msg = socket.recv_msg();

msg.to_str();


pub.transmit(msg, group);
let msg = sub.recv_msg();
```

### nng

```rust
let mut msg = Message::new();
msg.write(&[u8]);
socket.send(Message)

let msg = socket.recv();

String::from_utf8(msg.as_slice().to_vec()).unwrap();

pub.send(msg);
sub.set_opt::<Subscribe>(Vec<u8>);
let msg = sub.recv();
```