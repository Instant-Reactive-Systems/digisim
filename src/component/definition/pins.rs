#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Pins {
    pub input: Vec<String>,
    pub output: Vec<String>,
}

