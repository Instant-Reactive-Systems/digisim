use rassert_rs::rassert;

use super::Component;
use crate::sim::{Event, UserEvent, UserEventError};
use crate::circuit::Params;
use crate::util::*;

#[derive(Debug, Clone)]
pub struct Memory {
    // 0 - read
    // 1 - write
    rw_select: bool,
    chip_select: bool,
    address: Bits,
    data_in: Bits,
    data_out: Bits,

    storage: Vec<Bits>,
    changed: bool,
    delay: u32,
}

impl Component for Memory {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        if !self.changed { return None }

        // We only need to return Some since the address bits specify 
        // which storage word to access for read or write, not the actual
        // event, i.e. the event is just used a signal
        Some(vec![(0, false)])
    }

    fn update(&mut self, _event: Event) {
        let word_address = self.address.to_number();
        match self.rw_select {
            // Read
            false => {
                self.storage[word_address as usize] = self.data_in.clone();
            },
            // Write
            true => {
                self.data_out = self.storage[word_address as usize].clone();
            },
        }
    }

    fn set_pin(&mut self, pin: u32, event: Event) {
        // Skip if the pin being set isn't CS or if the chip select isn't set
        if pin != 0 || !self.chip_select { return }

        self.changed = true;

        let alen = self.address.len();
        let dlen = self.data_in.len();
        let end = alen + dlen;

        match pin as usize {
            0 => self.rw_select = event.value,
            1 => self.chip_select = event.value,
            n if (2..alen).contains(&n) => self.address.set_bit(n - 2, event.value),
            n if (alen..end).contains(&n) => self.data_in.set_bit(n - alen - 2, event.value),
            _ => {}
        }
    }

    fn get_state(&self) -> serde_json::Value {
        let words = self.storage.iter()
            .map(|word| word.to_vec())
            .collect::<Vec<Vec<bool>>>();

        serde_json::json!({"storage": words})
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            rw_select: false,
            chip_select: false,
            address: Bits::new(16),
            data_in: Bits::new(8),
            data_out: Bits::new(8),
            storage: vec![Bits::new(8); 16 * 1024],
            changed: false,
            delay: 1,
        }
    }
}

impl Memory {
    pub fn from_params(_params: Params) -> Self {
        Default::default()
    }
}

