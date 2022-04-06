use crate::circuit::Connection;
use super::Component;

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Circuit {
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
}
