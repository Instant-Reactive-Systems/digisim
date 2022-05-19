use crate::wasm;

#[wasm::wasm_bindgen]
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub max_delay: u32,
}

#[wasm::wasm_bindgen]
impl Settings {
    pub fn new(max_delay: u32) -> Self {
        Self {
            max_delay,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_delay: 2048,
        }
    }
}

