mod zmq;

use super::config::config;
use crate::config::config::{EdgeHubConfig, ServerProtocol};

pub async fn start(edge_hub_config: &EdgeHubConfig) {
    let protocol = config::EdgeHubConfig::get_hub_enabled_server_protocol(edge_hub_config);

    match protocol {
        ServerProtocol::ZMQ => start_zmq_server().await,
        ServerProtocol::WS => {}
        ServerProtocol::QUIC => {}
        ServerProtocol::MQTT => {}
    }
}

async fn start_zmq_server() {
    zmq::start().await;
}
