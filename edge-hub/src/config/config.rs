#[derive(Clone)]
pub struct EdgeHubConfig {
    server_protocol: ServerProtocol,
    client_protocol: ClientProtocol,
}

#[derive(Clone)]
pub enum ServerProtocol {
    ZMQ, WS, QUIC, MQTT
}

pub enum ClientProtocol {
    KAFKA, PULSAR
}

impl EdgeHubConfig {
    pub fn new(server_protocol: ServerProtocol, client_protocol: ClientProtocol) -> Self{
        EdgeHubConfig {
            server_protocol,
            client_protocol,
        }
    }

    pub fn get_hub_enabled_server_protocol(&self) -> &ServerProtocol {
        &self.server_protocol
    }
}