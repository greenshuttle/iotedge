mod internal;
mod metadata;
mod networks;
mod security;
mod shadow;
mod streaming;

use edge_model;
use internal::store::rusqlite;
#[cfg(feature = "mqtt")]
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

    // 4. inti mqtt networks.servers
    inti_mqtt_server();
}

fn inti_mqtt_server() {

}
