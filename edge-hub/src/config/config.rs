#[derive(Clone)]
pub struct EdgeHubConfig {
    protocol: Protocol,
}

#[derive(Clone)]
pub enum Protocol {
    ZMQ, WS, QUIC, MQTT
}

impl EdgeHubConfig {
    pub fn new(protocol: Protocol) -> Self{
        EdgeHubConfig {
            protocol,
        }
    }

    pub fn get_hub_enabled_protocol(&self) -> &Protocol {
        &self.protocol
    }
}