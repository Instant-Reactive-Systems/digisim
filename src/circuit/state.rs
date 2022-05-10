use std::collections::HashMap;
use super::Id;
use crate::wasm;

#[derive(Debug, Default)]
pub struct CircuitState {
    pub data: HashMap<Id, serde_json::Value>,
}

impl CircuitState {
    pub fn to_wasm_json(&self) -> wasm::JsValue {
        wasm::JsValue::from_serde(&self.data).unwrap()
    }
}

