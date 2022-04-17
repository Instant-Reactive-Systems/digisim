use crate::wasm;

#[wasm::wasm_bindgen]
#[derive(Debug)]
pub struct Config {
    pub max_delay: u32,
}

