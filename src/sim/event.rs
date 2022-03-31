use crate::circuit::Connector;

#[derive(Debug, Clone, Copy)]
pub struct Event {
    pub value: bool,
    pub src: Connector,
}

impl Event {
    pub fn new(value: bool, src: Connector) -> Self {
        Self {
            value,
            src,
        }
    }
}

