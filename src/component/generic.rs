use std::any::Any;
use std::fmt::{Debug, Formatter};
use super::{Component, ComponentDefinition};
use crate::sim::Event;

pub struct Generic {
    pub component_def: *const ComponentDefinition,
}

impl Component for Generic {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        unreachable!()
    }

    fn update(&mut self, _event: Event) {
        unreachable!()
    }

    fn set_pin(&mut self, _pin: u32, _event: Event) {
        unreachable!()
    }

    fn get_state(&self) -> serde_json::Value {
        unreachable!()
    }

    fn delay(&self) -> u32 {
        unreachable!()
    }

    fn is_source(&self) -> bool {
        false
    }

    fn is_output(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn reset(&mut self) {}
}

impl Debug for Generic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let component_def = unsafe { &*self.component_def };
        write!(f, "Generic component '{}', id {}", component_def.name, component_def.id)
    }
}

