use crate::component::definition::Component;
use super::Connection;
use crate::wasm;

#[wasm::wasm_bindgen]
#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircuitDefinition {
    pub(crate) id: i32,
    pub(crate) name: String,
    #[serde(rename = "description")] 
    pub(crate) desc: String,
    pub(crate) components: Vec<Component>,
    pub(crate) connections: Vec<Connection>,
}

