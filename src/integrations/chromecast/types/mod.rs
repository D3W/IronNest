use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoverRes {
    pub name: String,
    pub location: String,
    pub serial: String,
    pub mac: String,
    pub ipv4: String,
    pub port: String,
    pub services: Vec<String>,
}
