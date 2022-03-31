use super::Id;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CircuitState {
    pub data: HashMap<Id, serde_json::Value>,
}

