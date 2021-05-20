mod server;
mod shadow;
mod internal;
mod metadata;
mod security;

use internal::store::rusqlite;
use edge_model;
#[cfg(feature = "edge-mqtt")]
use mqtt_server::server as m_server;

#[tokio::main]
async fn main() {
    // 1. init internal.store
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
    tokio::spawn(async {
        shadow::device_client::create_mqtt_client().await;
    });

    // 4. inti mqtt server
    inti_mqtt_server();
}

fn inti_mqtt_server() {
    #[cfg(feature = "edge-mqtt")]
        m_server::say_hi();
}


