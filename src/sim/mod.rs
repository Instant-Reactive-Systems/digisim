mod event;
mod wheel;
mod config;
mod user_event;

pub use event::Event;
pub use user_event::{UserEvent, UserEventError};
pub use wheel::TimingWheel;
pub use config::Config;

use crate::Circuit;
use crate::circuit::{Registry, Connector, CircuitState};
use std::collections::HashSet;
use crate::wasm;

/// Simulation context
///
/// A single tick does not necessarily correspond to a single time unit.
#[wasm::wasm_bindgen]
#[derive(Debug)]
pub struct Simulation {
    pub(crate) circuit: Circuit,
    pub(crate) registry: Registry,
    pub(crate) wheel: TimingWheel,
    pub(crate) elapsed: u128,
}

#[wasm::wasm_bindgen]
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

    /// Initializes the simulation by inserting events from source components.
    pub fn init(&mut self) {
        for (&component_id, component) in self.circuit.components.iter() {
            if component.is_source() {
                if let Some(output) = component.evaluate() {
                    for (pin_id, value) in output {
                        let src = Connector::new(component_id, pin_id);
                        let event = Event::new(value, src);

                        self.wheel.schedule(0, event);
                    }
                }
            }
        }
    }

    /// Returns a JSON object containing the circuit state.
    pub fn circuit_state(&self) -> wasm::JsValue {
        let mut state = CircuitState::default();
        for id in self.circuit.output_components.iter() {
            let component = self.circuit.components.get(id).unwrap();
            state.data.insert(*id, component.get_state());
        }

        state.to_wasm_json()
    }

    pub fn set_circuit(&mut self, circuit: wasm::JsValue) {
        let circuit_def = circuit.into_serde().expect("Expected the circuit definition to be in correct format.");
        self.circuit = Circuit::from_definition(&self.registry, circuit_def).unwrap();
    }

    pub fn set_registry(&mut self, registry: wasm::JsValue) {
        let registry = registry.into_serde().expect("Expected the registry to be in correct format.");
        self.registry = registry;
    }

    pub fn update_registry(&mut self, definition: wasm::JsValue) {
        let component_def = definition.into_serde().expect("Expected the component definition to be in correct format");
        self.registry.insert(component_def);
    }

    pub fn insert_input_event(&mut self, event: wasm::JsValue) -> Result<(), String> {
        let user_event: UserEvent = event.into_serde().unwrap();
        let component = self.circuit.components.get(&user_event.component_id).unwrap();

        let events = match component.process_user_event(user_event) {
            Ok(events) => events,
            Err(e) => return Err(e.to_string()),
        };

        for event in events {
            self.wheel.schedule(component.delay(), event);
        }

        Ok(())
    }
}

