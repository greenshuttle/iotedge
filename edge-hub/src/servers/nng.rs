use log::info;
use nng::options::protocol::pubsub::Subscribe;
use nng::options::Options;
use nng::{Protocol, Socket};
use std::convert::TryInto;

pub struct Nng {}

pub async fn start() {
    info!("nng server starting...");
    let socket = Socket::new(Protocol::Pub0).unwrap();
    sub(&socket, "tcp://127.0.0.1:8080").await;
    tokio::spawn(accept_loop(socket));
    info!("nng server started.");
}

pub async fn sub(socket: &Socket, url: &str) {
    socket.dial(url).unwrap();

    info!("SUBSCRIBER: SUBSCRIBING TO ALL TOPICS");

    let all_topics = vec![];
    socket.set_opt::<Subscribe>(all_topics).unwrap();
}

pub async fn accept_loop(socket: Socket) {
    loop {
        let msg = socket.recv().unwrap();
        let subs = usize::from_le_bytes(msg[..].try_into().unwrap());
        info!("SUBSCRIBER: THERE ARE {} SUBSCRIBERS", subs);
    }
}
