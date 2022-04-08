use std::fmt::{Debug, Formatter};
use super::{Component, ComponentDefinition};
use crate::sim::Event;

pub struct Generic {
    pub component_def: *const ComponentDefinition,
}

impl Component for Generic {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        todo!()
    }

    fn update(&mut self, event: Event) {
        todo!()
    }

    fn set_pin(&mut self, pin: u32, event: Event) {
        todo!()
    }

    fn get_state(&self) -> serde_json::Value {
        todo!()
    }

    fn delay(&self) -> u32 {
        todo!()
    }

    fn is_source(&self) -> bool {
        todo!()
    }
}

impl Debug for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "Generic component with component definition id {}", (*self.component_def).id) }
    }
}

