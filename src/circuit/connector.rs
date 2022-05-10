use std::fmt::Display;

use super::Id;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct Connector {
    #[serde(rename = "componentId")] pub component: Id,
    pub pin: Id,
}

impl Connector {
    pub fn new(component_id: Id, pin_id: Id) -> Self {
        Self {
            component: component_id,
            pin: pin_id,
        }
    }
}

impl Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.component, self.pin)
    }
}

