use clap::{App, Arg};
use crate::config::config::{ServerProtocol, ClientProtocol, EdgeHubConfig};
use std::str::FromStr;

mod servers;
mod config;
mod clients;
mod internal;

#[tokio::main]
async fn main() {
    log4rs::init_file("edge-hub/log4rs.yaml", Default::default()).unwrap();

    let matches = App::new("Edge-Hub, a edge hub for edges.")
        .version("1.0")
        .about("Yang <zifeng.1024@gmail.com>")
        .arg(Arg::new("server-protocol")
            .long("server-protocol")
            .value_name("SERVER_PROTOCOL")
            .possible_values(&["WS", "ZMQ", "MQTT", "QUIC"])
            .default_value("ZMQ")
            .about("The server protocol that use between edge-core and edge-hub")
            .takes_value(true))
        .arg(Arg::new("client-protocol")
            .long("client-protocol")
            .value_name("CLIENT_PROTOCOL")
            .possible_values(&["KAFKA", "PULSAR"])
            .default_value("KAFKA")
            .about("The client protocol that use between edge-hub and user mq")
            .takes_value(true))
        .get_matches();

    let server_protocol = matches.value_of("server-protocol").unwrap();
    let server_protocol: ServerProtocol = ServerProtocol::from_str(server_protocol).unwrap();
    let client_protocol = matches.value_of("client-protocol").unwrap();
    let client_protocol: ClientProtocol = ClientProtocol::from_str(client_protocol).unwrap();
    let edge_hub_config = EdgeHubConfig::new(server_protocol, client_protocol);
    servers::start(&edge_hub_config).await;
    clients::start(&edge_hub_config);
}
