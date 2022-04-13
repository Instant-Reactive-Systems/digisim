use std::any::Any;

use crate::Component;
use crate::sim::Event;

#[derive(Debug)]
pub struct Clock {
    output: bool,

    cycle_delay: u32,
}

impl Component for Clock {
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
}

impl Clock {
    pub fn new(cycle_delay: u32) -> Self {
        Self {
            output: false,
            cycle_delay,
        }
    }
}

