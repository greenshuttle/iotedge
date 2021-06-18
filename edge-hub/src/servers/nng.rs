use log::info;
use nng::options::protocol::pubsub::Subscribe;
use nng::options::Options;
use nng::{Protocol, Socket, PipeEvent};
use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;

pub struct Nng {}

pub async fn start() {
    info!("nng server starting...");
    let socket = Socket::new(Protocol::Pub0).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    publish(&socket, "tcp://127.0.0.1:5555", count_clone).await;
    tokio::spawn(accept_loop(socket, count));
    info!("nng server started.");
}

pub async fn publish(socket: &Socket, url: &str, count_clone: Arc<AtomicUsize>) {

    socket.pipe_notify(move |_, ev| {
        match ev {
            PipeEvent::AddPost => count_clone.fetch_add(1, Ordering::Relaxed),
            PipeEvent::RemovePost => count_clone.fetch_sub(1, Ordering::Relaxed),
            _ => 0,
        };
    }).unwrap();

    socket.listen(url).expect("Nng bind 5555 failure");
}

pub async fn accept_loop(socket: Socket, count: Arc<AtomicUsize>) {
    loop {
        // Sleep for a little bit before sending the next message.
        thread::sleep(Duration::from_secs(3));

        // Load the number of subscribers and send the value across
        let data = count.load(Ordering::Relaxed) as u64;
        println!("PUBLISHER: SENDING {}", data);
        socket.send(data.to_le_bytes()).unwrap();
    }
}
