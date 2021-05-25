mod zmq;

use super::config::config;
use crate::config::config::Protocol;

pub fn start() {
    let edge_hub_config = config::EdgeHubConfig::new(config::Protocol::ZMQ);
    let protocol = config::EdgeHubConfig::get_hub_enabled_protocol(&edge_hub_config);

    match protocol {
        config::Protocol::ZMQ => start_zmq_server(),
        Protocol::WS => {}
        Protocol::QUIC => {}
        Protocol::MQTT => {}
    }
}

fn start_zmq_server() {
    zmq::start();
}