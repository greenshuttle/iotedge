mod store;

use store::rusqlite;

fn main() {
    // 1. init store
    let result = rusqlite::initial_store();
    match result {
        Ok(()) => println!(),
        Err(err) => println!("{}", err),
    }

    // 2. init core conn to the edge-hub
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send("hello world!", 0).unwrap();
    socket.disconnect("tcp://127.0.0.1:1234").unwrap();

    // 3. init mqtt conn from edge-hub to the cloud-pub
    // 4. inti http server
}
