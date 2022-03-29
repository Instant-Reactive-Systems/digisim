use std::{
    vec::Drain, 
    collections::{HashMap, HashSet}
};
use crate::{
    circuit::{ComponentId, Circuit}, 
    component::{Registry, Component},
    Connector,
};

pub type JsonValue = serde_json::Value;

pub struct Event {
    pub value: bool,
    pub src: Connector,
}

pub struct TimingWheel {
    max_delay: u32,
    current_time: u32,
    wheel: Vec<Vec<Event>>,
}

pub struct CircuitState {
    data: HashMap<ComponentId, JsonValue>,
}

pub struct Config {
    pub max_delay: u32,
}

pub struct Simulation {
    pub circuit: Option<Circuit>,
    pub registry: Option<Registry>,
    pub wheel: TimingWheel,
    pub elapsed: u128,
}

impl Simulation {
    pub fn new(config: Config) -> Self {
        Self {
            circuit: None,
            registry: None,
            wheel: TimingWheel::new(config.max_delay),
            elapsed: 0,
        }
    }

    pub fn tick(&mut self) {
        let mut circuit = self.circuit.unwrap();
        let (elapsed, events) = self.wheel.advance();
        self.elapsed += elapsed;
        let mut activity_set = HashSet::new();

        for event in events {
            let mut component = circuit.components.get_mut(&event.src.component_id);
            component.set_pin(event.src.pin_id, event.value);
            
            for connector in circuit.connections[event.src].iter() {
                let mut component = circuit.components.get_mut(&connector.component_id);
                component.set_pin(connector.pin_id, event.value);
                activity_set.insert(connector.component_id);
            }
        }

        for id in activity_set {
            let component = circuit.components.get(&id);
            
            if let Some(output_diff) = component.evaluate() {
                for (pin_id, value) in output_diff {
                    let src = Connector {
                        component_id: id,
                        pin_id,
                    };

                    let event = Event {
                        value,
                        src,
                    };

                    self.wheel.schedule(component.delay(), event);
                }
            }
        }
    }

    pub fn circuit_state(&self) -> CircuitState {
        let circuit = self.circuit.unwrap();
        let mut state = CircuitState::default();
        for (id, component) in circuit.components.iter() {
            state.data.insert(id, component.get_state());
        }

        state
    }

    pub fn set_circuit(&mut self, circuit: JsonValue) {
        unimplemented!()
    }

    pub fn set_registry(&mut self, registry: JsonValue) {
        unimplemented!()
    }

    pub fn update_registry(&mut self, definition: JsonValue) {
        unimplemented!()
    }

    pub fn insert_input_event(&mut self, event: JsonValue) {
        unimplemented!()
    }
}

impl TimingWheel {
    pub fn new(max_delay: u32) -> Self {
        Self {
            max_delay,
            current_time: 0,
            wheel: vec![Default::default(); max_delay],
        }
    }

    pub fn advance(&mut self) -> (u32, Drain<Event>) {
        let mut cnt = 0;
        while cnt != self.max_delay && self.wheel[self.current_time].is_empty() {
            self.current_time += 1;
            self.current_time %= self.max_delay;
            cnt += 1;
        }

        (cnt, self.wheel[self.current_time].drain(..))
    }

    pub fn schedule(&mut self, delay: u32, event: Event) {
        let scheduled_time = (self.current_time + delay) % self.max_delay;
        self.wheel[scheduled_time].push(event);
    }
}

impl Default for CircuitState {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

