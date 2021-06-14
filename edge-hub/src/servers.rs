mod zmq;

use crate::config::config::{EdgeHubConfig, ServerProtocol};
use crate::tenant::TenantId;
use crate::internal::store::queue::Queue;
use crate::message::Message;

pub struct EdgeCoreTwin {
    name: String,
    talent_id: TenantId,
    // every edge core use a single queue for cache messages
    message_queue: tokio::sync::mpsc::Sender<Message>,
    status: EdgeCoreStatus,
}

pub enum EdgeCoreStatus {
    ONLINE, OFFLINE
}


pub async fn start(edge_hub_config: &EdgeHubConfig) {
    let protocol = EdgeHubConfig::get_hub_enabled_server_protocol(edge_hub_config);

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

pub trait Server {
    fn start();

    fn accept();

    fn init();
}