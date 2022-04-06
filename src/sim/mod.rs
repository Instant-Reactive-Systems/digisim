mod event;
mod wheel;
mod config;
pub use event::Event;
pub use wheel::TimingWheel;
pub use config::Config;

use crate::Circuit;
use crate::circuit::{Registry, Connector, CircuitState};
use std::collections::HashSet;

/// Simulation context
///
/// A single tick does not necessarily correspond to a single time unit.
#[derive(Debug)]
pub struct Simulation {
    pub circuit: Circuit,
    pub registry: Registry,
    pub wheel: TimingWheel,
    pub elapsed: u128,
}

impl Simulation {
    /// Create a new simulation context.
    pub fn new(config: Config) -> Self {
        Self {
            circuit: Default::default(),
            registry: Default::default(),
            wheel: TimingWheel::new(config.max_delay),
            elapsed: 0,
        }
    }

    /// Processes the timing wheel.
    pub fn tick(&mut self) {
        let mut activity_set = HashSet::new();

        // Advance the timing wheel and record the elapsed time
        let (elapsed, events) = self.wheel.advance();
        self.elapsed += elapsed as u128;

        // Go through all the events, update the source component, 
        // set and schedule its dependent components
        for event in events {
            let component = self.circuit.components.get_mut(&event.src.component).unwrap();
            component.update(event);
            
            for to in self.circuit.connections[&event.src].iter() {
                let component = self.circuit.components.get_mut(&to.component).unwrap();
                component.set_pin(to.pin, event);
                activity_set.insert(to.component);
            }
        }

        // Go through all scheduled components
        for component_id in activity_set {
            let component = self.circuit.components.get(&component_id).unwrap();

            // If there were any changed outputs, schedule the event
            if let Some(output_diff) = component.evaluate() {
                for (pin_id, value) in output_diff {
                    let src = Connector::new(component_id, pin_id);
                    let event = Event::new(value, src);

                    self.wheel.schedule(component.delay(), event);
                }
            }
        }
    }

    /// Returns a JSON object containing the circuit state.
    pub fn circuit_state(&self) -> CircuitState {
        let mut state = CircuitState::default();
        for (&id, component) in self.circuit.components.iter() {
            state.data.insert(id, component.get_state());
        }

        state
    }

    pub fn set_circuit(&mut self, circuit: serde_json::Value) {
        unimplemented!()
    }

    pub fn set_registry(&mut self, registry: serde_json::Value) {
        unimplemented!()
    }

    pub fn update_registry(&mut self, definition: serde_json::Value) {
        unimplemented!()
    }

    pub fn insert_input_event(&mut self, event: serde_json::Value) {
        unimplemented!()
    }
}

