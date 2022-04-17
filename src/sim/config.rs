use crate::wasm;

#[wasm::wasm_bindgen]
#[derive(Debug)]
pub struct Config {
    pub max_delay: u32,
}

#[wasm::wasm_bindgen]
impl Config {
    pub fn new(max_delay: u32) -> Self {
        Self {
            max_delay,
        }
    }
}

