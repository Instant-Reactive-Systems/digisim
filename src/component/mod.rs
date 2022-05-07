pub mod definition;
mod tristate;
mod nand;
mod generic;
mod wiring;
mod switch;
mod ground;
mod source;
mod clock;
mod led;
mod display;

pub use definition::ComponentDefinition;
pub use tristate::Tristate;
pub use nand::Nand;
pub use generic::Generic;
pub use wiring::Wiring;
pub use switch::Switch;
pub use ground::Ground;
pub use source::Source;
pub use clock::Clock;
pub use led::Led;
pub use display::GenericDisplay;

use std::any::Any;
use std::fmt::Debug;
use crate::sim::{Event, UserEvent, UserEventError};

/// Trait that all components implement.
pub trait Component: Any + Debug {
    /// Initial evaluation for the component.
    fn initial_evaluate(&self) -> Option<Vec<(u32, bool)>>;

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

    /// Checks if the component is an output component.
    fn is_output(&self) -> bool;

    /// Cast to Any.
    fn as_any(&self) -> &dyn Any;

    /// Processes a user event.
    fn process_user_event(&self, _user_event: UserEvent) -> Result<Vec<Event>, UserEventError> {
        unimplemented!()
    }
}

