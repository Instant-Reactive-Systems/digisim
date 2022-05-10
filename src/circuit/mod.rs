mod id;
mod connector;
mod connection;
mod state;
pub mod registry;
mod definition;
mod params;
pub use id::Id;
pub use connector::Connector;
pub use connection::Connection;
pub use state::CircuitState;
pub use registry::Registry;
pub use definition::CircuitDefinition;
pub use params::Params;

use std::collections::HashMap;
use crate::component::definition::ComponentKind;
use crate::component::{self, Component, ComponentDefinition, Generic, Wiring};
use DefinitionError::*;
use self::registry::PREBUILT_REGISTRY;
use rassert_rs::rassert;

/// A self-contained collection of all components and its wiring.
#[derive(Debug, Default)]
pub struct Circuit {
    pub components: HashMap<Id, Box<dyn Component>>,
    pub output_components: Vec<Id>,
    pub connections: HashMap<Connector, Vec<Connector>>,

    pub rerouted_defs: HashMap<Id, ComponentDefinition>,

    /// Maps components IDs to their corresponding component definition ID
    ///
    /// Note: Used only during definition building.
    definition_mapping: HashMap<Id, i32>,
}

impl Circuit {
    pub fn from_definition(registry: &Registry, mut circuit_def: CircuitDefinition) -> Result<Self, DefinitionError> {
        // 1.) Iterate through the components in the circuit
        // 2.) Process only non-transparent components, and put transparent ones into a separate
        // list
        // 3.) Then iterate through transparent components and recursively process inner transparent 
        // components by remapping their connections down to their concrete components
        let mut circuit = Circuit::default();
        let mut transparent_components = Vec::new();

        for &component in circuit_def.components.iter() {
            let component_def = registry.get_definition(component.def_id)?;
            let ctx = Context {
                component,
                component_def,
                params: circuit_def.params.as_ref(),
                registry,
            };

            match component_def.kind {
                ComponentKind::Builtin => circuit.process_builtin(ctx)?,
                ComponentKind::Compiled => circuit.process_compiled(ctx)?,
                ComponentKind::Functional => circuit.process_functional(ctx)?,
                ComponentKind::Transparent => {
                    // Insert transparent component ahead of time for IDs to work
                    circuit.components.insert(ctx.component.id, ctx.component_def.instantiate(Params::default()));
                    transparent_components.push(ctx)
                },
            }
        }

        // Process transparent components finally
        for transparent in transparent_components {
            circuit.process_transparent(transparent)?;
        }

        // Insert wiring component and add it to circuit definition's connections for top-level
        // components
        let mut wiring = Wiring::default();
        let mut count = 0;
        for component in circuit_def.components.iter() {
            let def = registry.get_definition(component.def_id).unwrap();

            let a = def.pins.input.len();
            let b = a + def.pins.output.len();
            for pin in a..b {
                let from = Connector { component: component.id, pin: pin as u32 };
                let mut to = vec![Connector { component: Id::MAX, pin: count as u32 }];
                wiring.add_output(from);

                if let Some(found) = circuit_def.connections.iter_mut().find(|x| x.from == from) {
                    found.to.append(&mut to);
                } else {
                    circuit_def.connections.push(Connection { from, to });
                }
                count += 1;
            }
        }
        circuit.components.insert(Id::MAX, Box::new(wiring));
        circuit.output_components.push(Id::MAX);

        // Insert all top-level connections
        for connection in circuit_def.connections.iter() {
            let rerouted_connections = circuit.reroute_connection(connection)?;

            rerouted_connections.into_iter().for_each(|conn| {
                circuit.connections.insert(conn.from, conn.to);
            });
        }

        // Insert the internal connections of a transparent component
        for component_def in circuit.rerouted_defs.values() {
            for connection in component_def.circuit.as_ref().unwrap().connections.iter() {
                let rerouted_connections = circuit.reroute_connection(connection)?;

                rerouted_connections.into_iter().for_each(|conn| {
                    circuit.connections.insert(conn.from, conn.to);
                });
            }
        }

        // Discard definition mapping
        circuit.definition_mapping.clear();

        Ok(circuit)
    }

    fn process_builtin(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        rassert!(!self.components.contains_key(&ctx.component.id), ComponentIdAlreadyTaken(ctx.component.id));

        let params = if let Some(params) = ctx.params {
            params.get(&ctx.component.id).cloned().unwrap_or_default()
        } else {
            Default::default()
        };
        let component = ctx.component_def.instantiate(params);
        if component.is_output() {
            self.output_components.push(ctx.component.id);
        }
        self.components.insert(ctx.component.id, component);
        self.definition_mapping.insert(ctx.component.id, ctx.component_def.id);

        Ok(())
    }

    fn process_compiled(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        todo!()
    }

    fn process_functional(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        todo!()
    }

    fn process_transparent(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        let mut transparent_components = Vec::new();

        // Reroute the component definition
        let rerouted_def = ctx.component_def.reroute_component_def(self.components.len() as u32);
        let rerouted_circuit = rerouted_def.circuit.as_ref().ok_or(InvalidTransparentComponent("No circuit field".into()))?;
        self.rerouted_defs.insert(ctx.component.id, rerouted_def.clone());

        // Store the definition mapping to the ID
        self.definition_mapping.insert(ctx.component.id, ctx.component_def.id);

        // Process concrete components and defer transparent ones
        for &component in rerouted_circuit.components.iter() {
            let component_def = ctx.registry.get_definition(component.def_id)?;
            let ctx = Context {
                component,
                component_def,
                params: rerouted_circuit.params.as_ref(),
                registry: ctx.registry,
            };

            match component_def.kind {
                ComponentKind::Builtin => self.process_builtin(ctx)?,
                ComponentKind::Compiled => self.process_compiled(ctx)?,
                ComponentKind::Functional => self.process_functional(ctx)?,
                ComponentKind::Transparent => {
                    // Insert transparent component ahead of time for IDs to work
                    self.components.insert(ctx.component.id, ctx.component_def.instantiate(Default::default()));
                    transparent_components.push(ctx)
                },
            }
        }

        // Insert and process inner transparent components
        for transparent in transparent_components {
            self.process_transparent(transparent)?;
        }

        Ok(())
    }

    /// Reroutes the connector to the first connected builtin component.
    fn reroute_to_concrete(&self, connector: Connector) -> Result<Vec<Connector>, DefinitionError> {
        let mut rerouted_connectors = Vec::new();
        self.reroute_to_concrete_impl(connector, &mut rerouted_connectors)?;

        Ok(rerouted_connectors)
    }

    /// Reroutes the connector to the first connected concrete component.
    fn reroute_to_concrete_impl(&self, connector: Connector, rerouted_connectors: &mut Vec<Connector>) -> Result<(), DefinitionError> {
        let component = self.components.get(&connector.component).ok_or(InvalidConnector(connector))?;

        if let Some(_) = get_transparent(component) {
            let rerouted_def = self.rerouted_defs.get(&connector.component).unwrap();
            let pin_mapping = rerouted_def.pin_mapping.as_ref().unwrap();
            let input = pin_mapping.input.iter();
            let output = pin_mapping.output.iter();
            let mut pins = input.chain(output);
            
            let connectors = pins.nth(connector.pin as usize).ok_or(InvalidConnector(connector))?;
            for connector in connectors {
                self.reroute_to_concrete_impl(*connector, rerouted_connectors)?;
            }

            return Ok(());
        }

        // Add concrete component
        rerouted_connectors.push(connector);
        Ok(())
    }

    /// Wires Clock components back into itself so that events repeat.
    fn wire_clocks_into_itself(&self, connections: &mut Vec<Connection>) {
        connections.iter_mut()
            .filter(|x| {
                let def_id = self.definition_mapping[&x.from.component];
                
                // If component is prebuilt and definition ID is the CLOCK_ID
                PREBUILT_REGISTRY.with(|reg| {
                    let entry = reg.data.get(&def_id);

                    if let Some(entry) = entry {
                        entry.def.id == registry::CLOCK_ID
                    } else {
                        false
                    }
                })
            })
            .for_each(|x| {
                // Connect to itself
                let to_self = Connector { component: x.from.component, pin: 1 };
                x.to.push(to_self);
            });
    }

    fn reroute_connection(&self, connection: &Connection) -> Result<Vec<Connection>, DefinitionError> {
        let from = self.reroute_to_concrete(connection.from)?;
        let to: Vec<Connector> = connection.to.iter().map(|x| {
            self.reroute_to_concrete(*x)
        }).collect::<Result<Vec<Vec<Connector>>, _>>()?.into_iter().flatten().collect();

        // Each 'from' connector needs to be connected to all the 'to' connectors
        let mut connections: Vec<Connection> = from.iter()
            .map(|from| Connection { from: from.clone(), to: to.clone() })
            .collect();
        self.wire_clocks_into_itself(&mut connections);
        
        Ok(connections)
    }
}

fn get_transparent(component: &Box<dyn Component>) -> Option<&Generic> {
    if let Some(generic) = component.as_any().downcast_ref::<Generic>() {
        if unsafe { (*generic.component_def).kind == ComponentKind::Transparent } {
            return Some(generic);
        }
    }

    None
}

#[derive(Debug)]
struct Context<'a> {
    component: component::definition::Component,
    component_def: &'a ComponentDefinition,
    params: Option<&'a HashMap<Id, Params>>,
    registry: &'a Registry,
}

#[derive(Debug, thiserror::Error)]
pub enum DefinitionError {
    #[error("Component with id {0} already exists, cannot take its place.")]
    ComponentIdAlreadyTaken(u32),

    #[error("Encountered a registry error.")]
    RegistryError(#[from] registry::RegistryError),

    #[error("Invalid transparent component found. Context: {0}")]
    InvalidTransparentComponent(String),

    #[error("Invalid connector {0} found in circuit connections.")]
    InvalidConnector(Connector),
}


#[cfg(test)]
mod tests {
    use crate::{component::ComponentDefinition, Circuit};
    use super::{CircuitDefinition, Registry};

    #[test]
    fn nand_gate() {
        let mut registry = Registry::default();

        let def = include_str!("../../tests/assets/and_gate_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        registry.insert(parsed);
        let def = include_str!("../../tests/assets/not_gate_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        registry.insert(parsed);

        let def = include_str!("../../tests/assets/nand_gate_circuit.json");
        let parsed: CircuitDefinition = serde_json::from_str(def).unwrap();
        let circuit = Circuit::from_definition(&registry, parsed).unwrap();

        for (id, component) in circuit.components.iter() {
            println!("{}. Component: {:?}", id, component);
        }
    }

    #[test]
    fn ab_inverted() {
        let mut registry = Registry::default();

        let def = include_str!("../../tests/assets/and_gate_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        registry.insert(parsed);
        let def = include_str!("../../tests/assets/not_gate_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        registry.insert(parsed);
        let def = include_str!("../../tests/assets/ab_inverted_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        registry.insert(parsed);

        let def = include_str!("../../tests/assets/ab_inverted_on_not_circuit.json");
        let parsed: CircuitDefinition = serde_json::from_str(def).unwrap();
        let circuit = Circuit::from_definition(&registry, parsed).unwrap();

        println!("AB inverted");
        for (id, component) in circuit.components.iter() {
            let rerouted_def = circuit.rerouted_defs.get(id);
            if let Some(def) = rerouted_def {
                println!("{}. Component: {:?}", id, component);
                println!("{:?}", def.pin_mapping);
                println!("{:?}", def.circuit.as_ref().unwrap().connections);
            } else {
                println!("{}. Component: {:?}", id, component);
            }

        }

        for (from, to) in circuit.connections.iter() {
            println!("Connection: (from: {:?}, to: {:?})", from, to);
        }
    }
}



