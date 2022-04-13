use std::any::Any;
use super::Component;
use crate::circuit::Connector;
use crate::sim::Event;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Wiring {
    pub outputs: HashMap<Connector, bool>,
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
        let mut outputs = Vec::new();

        for (at, value) in self.outputs.iter() {
            let output = serde_json::json!({
                "connector": at,
                "value": value,
            });
            outputs.push(output);
        }

        serde_json::to_value(outputs).unwrap()
    }

    fn delay(&self) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_state_ok() {
        let mut wiring = Wiring::default();
        wiring.outputs.insert(Connector::new(0, 0), true);
        wiring.outputs.insert(Connector::new(0, 1), false);
        let state = wiring.get_state();

        // Sort to make test testable
        let mut vec = state.as_array().unwrap().clone();
        vec.sort_by(|a, b| {
            a["connector"]["componentId"].as_u64().unwrap()
                .cmp(&b["connector"]["componentId"].as_u64().unwrap())
        });
        let state = serde_json::to_value(vec).unwrap();

        let expected = serde_json::json!([
            {
                "connector": {
                    "componentId": 0u32,
                    "pin": 0u32,
                },
                "value": true,
            },
            {
                "connector": {
                    "componentId": 0u32,
                    "pin": 1u32,
                },
                "value": false,
            },
        ]);

        assert_eq!(state, expected);
    }
}

