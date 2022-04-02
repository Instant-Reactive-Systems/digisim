
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
pub struct Connector {
    #[serde(rename = "componentId")] pub component: u32,
    pub pin: u32,
}

