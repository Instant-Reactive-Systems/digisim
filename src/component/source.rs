use std::any::Any;
use super::Component;
use crate::{sim::Event, circuit::Params};

#[derive(Debug, Clone, Default)]
pub struct Source;

impl Component for Source {
    fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
        Some(vec![(0, true)])
    }

    fn update(&mut self, _event: Event) {}

    fn set_pin(&mut self, _pin: u32, _event: Event) {
        // set_pin is not implemented for source components
        unreachable!()
    }

    fn get_state(&self) -> serde_json::Value {
        unimplemented!("Source does not implement get_state since it is not an output component.");
    }

    fn delay(&self) -> u32 {
        // Never called since this source component is only meant to be called when
        // the simulation is initialized
        unreachable!()
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

    fn reset(&mut self) {}
}

impl Source {
    pub fn from_params(_: Params) -> Self {
        Default::default()
    }
}

