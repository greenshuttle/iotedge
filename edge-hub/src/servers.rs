mod zmq;

use super::config::config;
use crate::config::config::{ServerProtocol, EdgeHubConfig};

pub fn start(edge_hub_config: &EdgeHubConfig) {

    let protocol = config::EdgeHubConfig::get_hub_enabled_server_protocol(edge_hub_config);

    match protocol {
        ServerProtocol::ZMQ => start_zmq_server(),
        ServerProtocol::WS => {}
        ServerProtocol::QUIC => {}
        ServerProtocol::MQTT => {}
    }
}

fn start_zmq_server() {
    zmq::start();
}