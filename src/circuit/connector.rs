use super::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connector {
    pub component: Id,
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

