use crate::{util::*, circuit::Params};
use super::Component;
use crate::sim::Event;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct GenericDisplay {
    enable: bool,
    // and another invisible pin 'value'
    address_x: Bits,
    address_y: Bits,

    pixels: Vec<Bits>,
}

impl Component for GenericDisplay {
    fn initial_evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
    }

    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        None
	}

	fn update(&mut self, event: Event) {}

	fn set_pin(&mut self, pin: u32, event: Event) {
        // Skip if not enabled
        if pin != 0 && self.enable == true { return }

        match pin as usize {
            0 => self.enable = event.value,
            1 => {
                let y = self.address_y.to_number() as usize;
                let x = self.address_x.to_number() as usize;
                self.pixels[y].set_bit(x, event.value);
            },
            n if (2..self.begin_address_x()).contains(&n) => self.address_x.set_bit(n, event.value),
            n if (self.begin_address_x()..self.begin_address_y()).contains(&n) => self.address_x.set_bit(n, event.value),
            _ => {}
        }
	}

	fn get_state(&self) -> serde_json::Value {
        let pixels = self.pixels.iter()
            .map(|pixel| pixel.to_vec())
            .collect::<Vec<Vec<bool>>>();

        serde_json::json!({
            "pixels": pixels,
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    
    fn reset(&mut self) {
        self.enable = false;
        self.address_x.clear();
        self.address_y.clear();
        self.pixels.iter_mut().for_each(|bits| bits.clear());
    }
}

impl Default for GenericDisplay {
    fn default() -> Self {
        Self {
            enable: false,
            address_x: Bits::new(32),
            address_y: Bits::new(32),
            pixels: vec![Bits::new(32); 32],
        }
    }
}

impl GenericDisplay {
    pub fn from_params(_params: Params) -> Self {
        Default::default()
    }

    fn begin_address_x(&self) -> usize {
        self.address_x.len()
    }

    fn begin_address_y(&self) -> usize {
        self.address_y.len()
    }
}

