use super::Component;
use crate::circuit::Connector;
use crate::sim::Event;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Wiring {
    outputs: HashMap<Connector, bool>,
}

impl Component for Wiring {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
    }

    fn update(&mut self, _event: Event) {}

    fn set_pin(&mut self, _pin: u32, event: Event) {
        self.outputs.insert(event.src, event.value);
    }

    fn get_state(&self) -> serde_json::Value {
        todo!()
    }

    fn delay(&self) -> u32 {
        1
    }

    fn is_source(&self) -> bool {
        false
    }
}

