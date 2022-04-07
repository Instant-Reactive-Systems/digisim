pub mod definition;
pub mod tristate;
pub mod nand;
pub mod generic;
pub mod wiring;
pub use definition::ComponentDefinition;
pub use tristate::Tristate;
pub use nand::Nand;
pub use generic::Generic;
pub use wiring::Wiring;

use std::any::Any;
use std::fmt::Debug;
use crate::sim::Event;

/// Trait that all components implement.
pub trait Component: Any + Debug {
    /// Evaluates the current state of the component
    /// and returns changed outputs, if any.
    fn evaluate(&self) -> Option<Vec<(u32, bool)>>;
    
    /// Updates the output according to the event.
    fn update(&mut self, event: Event);

    /// Sets the specified pin to the specified value.
    fn set_pin(&mut self, pin: u32, event: Event);

    /// Gets the current state of the component.
    fn get_state(&self) -> serde_json::Value;

    /// Gets the delay of the component.
    fn delay(&self) -> u32;

    /// Checks if the component is a source component.
    fn is_source(&self) -> bool;
}

