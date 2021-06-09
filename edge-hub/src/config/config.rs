use std::str::FromStr;

#[derive(Clone)]
pub struct EdgeHubConfig {
    server_protocol: ServerProtocol,
    client_protocol: ClientProtocol,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerProtocol {
    ZMQ,
    WS,
    QUIC,
    MQTT,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClientProtocol {
    KAFKA,
    PULSAR,
}

impl FromStr for ServerProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ZMQ" => Ok(ServerProtocol::ZMQ),
            "WS" => Ok(ServerProtocol::WS),
            "QUIC" => Ok(ServerProtocol::QUIC),
            "MQTT" => Ok(ServerProtocol::MQTT),
            _ => Err(()),
        }
    }
}

impl FromStr for ClientProtocol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "KAFKA" => Ok(ClientProtocol::KAFKA),
            "PULSAR" => Ok(ClientProtocol::PULSAR),
            _ => Err(()),
        }
    }
}

impl EdgeHubConfig {
    pub fn new(server_protocol: ServerProtocol, client_protocol: ClientProtocol) -> Self {
        EdgeHubConfig {
            server_protocol,
            client_protocol,
        }
    }

    pub fn get_hub_enabled_server_protocol(&self) -> &ServerProtocol {
        &self.server_protocol
    }

    pub fn get_hub_enabled_client_protocol(&self) -> &ClientProtocol {
        &self.client_protocol
    }
}
