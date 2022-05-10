use std::any::Any;

use crate::Component;
use crate::circuit::Params;
use crate::sim::Event;

#[derive(Debug, Default)]
pub struct Clock {
    output: bool,

    cycle_delay: u32,
}

impl Component for Clock {
    fn initial_evaluate(&self) -> Option<Vec<(u32, bool)>> {
        // We need to kickstart the repetition cycle by emitting a signal into the simulator
        Some(vec![(0, false)])
    }

    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        Some(vec![(0, !self.output)])
    }

    fn update(&mut self, event: Event) {
        self.output = event.value;
    }

    fn set_pin(&mut self, _pin: u32, _event: Event) {}

    fn get_state(&self) -> serde_json::Value {
        unimplemented!("Clock does not implement get_state since it is not an output component.");
    }

    fn delay(&self) -> u32 {
        self.cycle_delay
    }

    fn is_source(&self) -> bool {
        true
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

    fn reset(&mut self) {
        self.output = false;
    }
}

impl Clock {
    pub fn new(cycle_delay: u32) -> Self {
        Self {
            output: false,
            cycle_delay,
        }
    }

    pub fn from_params(params: Params) -> Self {
        let cycle_delay = if let Some(param) = params.get("delay") {
            param.as_u64().unwrap() as u32
        } else {
            1
        };

        Self {
            cycle_delay,
            ..Default::default()
        }
    }
}

