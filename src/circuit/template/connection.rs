use super::Connector;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Connection {
    pub from: Connector,
    pub to: Vec<Connector>,
}
