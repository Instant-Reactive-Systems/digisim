use std::any::Any;
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

    fn is_output(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Debug for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let component_def = unsafe { &*self.component_def };
        write!(f, "Generic component '{}', id {}", component_def.name, component_def.id)
    }
}

