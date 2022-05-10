use std::any::Any;
use super::Component;
use crate::circuit::Connector;
use crate::sim::Event;

#[derive(Debug, Default)]
pub struct Wiring {
    pub outputs: Vec<Connector>,
    pub values: Vec<bool>,
}

impl Component for Wiring {
    fn initial_evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
    }

    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
    }

    fn update(&mut self, _event: Event) {}

    fn set_pin(&mut self, pin: u32, event: Event) {
        self.values[pin as usize] = event.value;
    }

    fn get_state(&self) -> serde_json::Value {
        let mut outputs = Vec::new();

        for (at, value) in self.outputs.iter().zip(self.values.iter()) {
            let output = serde_json::json!({
                "connector": at,
                "value": value,
            });
            outputs.push(output);
        }

        serde_json::to_value(outputs).unwrap()
    }

    fn delay(&self) -> u32 {
        // Wiring is an output component, and as such, does not propagate signals
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn reset(&mut self) {
        self.values.iter_mut().for_each(|x| *x = false);
    }
}

impl Wiring {
    pub fn add_output(&mut self, connector: Connector) {
        self.outputs.push(connector);
        self.values.push(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_state_ok() {
        let mut wiring = Wiring::default();
        wiring.add_output(Connector::new(0, 0));
        wiring.add_output(Connector::new(0, 1));
        let state = wiring.get_state();

        let expected = serde_json::json!([
            {
                "connector": {
                    "componentId": 0u32,
                    "pin": 0u32,
                },
                "value": false,
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

