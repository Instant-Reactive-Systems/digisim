use crate::circuit::Connector;

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct PinMapping {
    pub input: Vec<Vec<Connector>>,
    pub output: Vec<Vec<Connector>>,
}

