pub mod kafka;

use super::config;

pub fn start() {
    let edge_hub_config = config::EdgeHubConfig::new(config::config::ClientProtocol::KAFKA);
    let protocol = config::EdgeHubConfig::get_hub_enabled_server_protocol(&edge_hub_config);

    match protocol {
        config::Protocol::ZMQ => start_zmq_server(),
        Protocol::WS => {}
        Protocol::QUIC => {}
        Protocol::MQTT => {}
    }
}