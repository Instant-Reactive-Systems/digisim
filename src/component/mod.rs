mod registry;

pub use registry::Registry;

use crate::sim::{TimingWheel, JsonValue};
use std::any::Any;
use std::fmt::Debug;

/// Trait that all components implement.
pub trait Component: Any + Debug {
    /// Evaluates the current state of the component
    /// and returns changed outputs.
    fn evaluate(&self) -> Option<Vec<(u32, bool)>>;

    /// Sets the specified pin to the specified value.
    fn set_pin(&mut self, pin: u32, value: bool);

    /// Emits signals from changed outputs into the provided Wiring.
    fn schedule(&self, id: u32, sim: &mut TimingWheel);

    /// Gets the current state of the component.
    fn get_state(&self) -> JsonValue;

    /// Gets the current state of the component.
    fn delay(&self) -> u32;
}

