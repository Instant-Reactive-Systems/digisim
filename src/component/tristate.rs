use super::Component;
use crate::sim::Event;

#[derive(Debug, Clone, Default)]
pub struct Tristate {
    a: bool,
    b: bool,
    c: bool,

    delay: u32,
}

impl Component for Tristate {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        // Component is disconnected (z-state)
        if self.b == false || self.a == self.c {
            return None;
        }

        Some(vec![(2, self.a)])
    }

    fn update(&mut self, event: Event) {
        match event.src.pin {
            2 => self.c = event.value,
            _ => {}
        }
    }

    fn set_pin(&mut self, pin: u32, value: bool) {
        match pin {
            0 => self.a = value,
            1 => self.b = value,
            _ => {}
        }
    }

    fn get_state(&self) -> serde_json::Value {
        todo!()
    }

    fn delay(&self) -> u32 {
        self.delay
    }
}

