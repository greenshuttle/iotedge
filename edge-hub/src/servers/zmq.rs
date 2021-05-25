use std::thread;
use std::time::Duration;

pub fn start() {
    let context = zmq::Context::new();
    let responder = context.socket((zmq::REP)).unwrap();
    responder.bind("tcp://*:5555");

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}