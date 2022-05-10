use std::any::Any;
use super::Component;
use crate::circuit::Params;
use crate::sim::Event;

#[derive(Debug, Clone, Default)]
pub struct Tristate {
    a: bool,
    b: bool,
    output: bool,

    delay: u32,
    initted: bool,
}

impl Component for Tristate {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        // Component is disconnected (z-state)
        if self.initted && (self.b == false || self.a == self.output) {
            return None;
        }

        Some(vec![(2, self.a)])
    }

    fn update(&mut self, event: Event) {
        self.initted = true;

        match event.src.pin {
            2 => self.output = event.value,
            _ => {}
        }
    }

    fn set_pin(&mut self, pin: u32, event: Event) {
        match pin {
            0 => self.a = event.value,
            1 => self.b = event.value,
            _ => {}
        }
    }

    fn get_state(&self) -> serde_json::Value {
        unreachable!("Tristate does not implement get_state as it is not an output component.");
    }

    fn delay(&self) -> u32 {
        self.delay
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

    fn reset(&mut self) {
        self.a = false;
        self.b = false;
        self.output = false;
        self.initted = false;
    }
}

impl Tristate {
    pub fn from_params(params: Params) -> Self {
        let delay = if let Some(param) = params.get("delay") {
            param.as_u64().unwrap() as u32
        } else {
            1
        };

        Self {
            delay,
            ..Default::default()
        }
    }
}

