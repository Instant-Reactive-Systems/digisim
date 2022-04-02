use super::{Component, Connection};

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Circuit {
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
}
