use super::Connector;

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Connection {
    pub from: Connector,
    pub to: Vec<Connector>,
}

