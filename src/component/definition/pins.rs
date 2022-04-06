use crate::circuit::Connector;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Pins {
    pub input: Vec<Connector>,
    pub output: Vec<Connector>,
}

