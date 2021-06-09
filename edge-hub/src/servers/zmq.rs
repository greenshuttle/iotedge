use std::thread;
use std::time::Duration;
use log::info;
use zmq::Socket;

pub async fn start() {
    info!("zmq server starting...");
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    bind(&responder).await;
    tokio::spawn(accept_loop(responder));
}

pub async fn bind(socket: &Socket) -> &Socket {
    socket.bind("tcp://*:5555").expect("Zmq bind 5555 failure");
    socket
}

pub async fn accept_loop(responder: Socket) {
    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}