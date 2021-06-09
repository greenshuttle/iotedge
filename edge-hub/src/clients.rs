pub mod kafka;
use super::config::config::{ClientProtocol, EdgeHubConfig};
use log::info;

pub fn start(edge_hub_config: &EdgeHubConfig) {
    let protocol = EdgeHubConfig::get_hub_enabled_client_protocol(edge_hub_config);

    match protocol {
        ClientProtocol::KAFKA => {
            info!("Kafka client is enabled!")
        }
        ClientProtocol::PULSAR => {
            info!("Pulsar client is enabled!")
        }
    }
}
