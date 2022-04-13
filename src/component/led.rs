use std::any::Any;
use super::Component;
use crate::sim::Event;

#[derive(Debug, Clone, Default)]
pub struct Led {
    value: bool,
}

impl Component for Led {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
    }

    fn update(&mut self, _event: Event) {}

    fn set_pin(&mut self, _pin: u32, event: Event) {
        self.value = event.value;
    }

    fn get_state(&self) -> serde_json::Value {
        serde_json::json!({
            "connector": 0u32,
            "value": self.value,
        })
    }

    fn delay(&self) -> u32 {
        // Never called since signal propagation ends with output components
        unreachable!()
    }

    fn is_source(&self) -> bool {
        false
    }

    fn is_output(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

