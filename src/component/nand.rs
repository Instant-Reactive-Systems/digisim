use super::Component;
use crate::sim::Event;

#[derive(Debug, Clone, Default)]
pub struct Nand {
    a: bool,
    b: bool,
    output: bool,

    delay: u32,
}

impl Component for Nand {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        let new = !(self.a & self.b);
        if new == self.output {
            return None;
        }

        Some(vec![(2, new)])
    }

    fn update(&mut self, event: Event) {
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
        todo!()
    }

    fn delay(&self) -> u32 {
        self.delay
    }

    fn is_source(&self) -> bool {
        false
    }
}

