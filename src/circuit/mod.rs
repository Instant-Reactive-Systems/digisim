mod id;
mod connector;
mod connection;
mod state;
pub mod registry;
mod definition;
pub use id::Id;
pub use connector::Connector;
pub use connection::Connection;
use rassert_rs::rassert;
pub use state::CircuitState;
pub use registry::Registry;
pub use definition::CircuitDefinition;

use std::collections::HashMap;
use crate::component::definition::ComponentKind;
use crate::component::{self, Component, ComponentDefinition, Generic};
use registry::RegistryError;

/// A self-contained collection of all components and its wiring.
#[derive(Debug, Default)]
pub struct Circuit {
    pub components: HashMap<Id, Box<dyn Component>>,
    pub output_components: Vec<Id>,
    pub connections: HashMap<Connector, Vec<Connector>>,

    rerouted_defs: HashMap<Id, ComponentDefinition>,
}

impl Circuit {
    pub fn from_definition(registry: &Registry, circuit_def: CircuitDefinition) -> Result<Self, DefinitionError> {
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
                registry,
            };

            match component_def.kind {
                ComponentKind::Builtin => circuit.process_builtin(ctx)?,
                ComponentKind::Compiled => circuit.process_compiled(ctx)?,
                ComponentKind::Functional => circuit.process_functional(ctx)?,
                ComponentKind::Transparent => {
                    // Insert transparent component ahead of time for IDs to work
                    circuit.components.insert(ctx.component.id, ctx.component_def.instantiate());
                    transparent_components.push(ctx)
                },
            }
        }

        // Process transparent components finally
        for transparent in transparent_components {
            circuit.process_transparent(transparent)?;
        }

        // Insert all top-level connections
        for connection in circuit_def.connections.iter() {
            let rerouted_connections = circuit.reroute_connection(connection, registry)?;

            rerouted_connections.into_iter().for_each(|conn| {
                circuit.connections.insert(conn.from, conn.to);
            });
        }

        // Insert the internal connections of a transparent component
        for component_def in circuit.rerouted_defs.values() {
            for connection in component_def.circuit.as_ref().unwrap().connections.iter() {
                let rerouted_connections = circuit.reroute_connection(connection, registry)?;

                rerouted_connections.into_iter().for_each(|conn| {
                    circuit.connections.insert(conn.from, conn.to);
                });
            }
        }

        Ok(circuit)
    }

    fn process_builtin(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        rassert!(!self.components.contains_key(&ctx.component.id), DefinitionError::ComponentIdAlreadyTaken(ctx.component.id));

        let component = ctx.component_def.instantiate();
        if component.is_output() {
            self.output_components.push(ctx.component.id);
        }
        self.components.insert(ctx.component.id, component);

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
        let rerouted_circuit = rerouted_def.circuit.as_ref()
            .ok_or(DefinitionError::InvalidTransparentComponent("No circuit field".into()))?;
        self.rerouted_defs.insert(ctx.component.id, rerouted_def.clone());

        // Process concrete components and defer transparent ones
        for &component in rerouted_circuit.components.iter() {
            let component_def = ctx.registry.get_definition(component.def_id)?;
            let ctx = Context {
                component,
                component_def,
                registry: ctx.registry,
            };

            match component_def.kind {
                ComponentKind::Builtin => self.process_builtin(ctx)?,
                ComponentKind::Compiled => self.process_compiled(ctx)?,
                ComponentKind::Functional => self.process_functional(ctx)?,
                ComponentKind::Transparent => {
                    // Insert transparent component ahead of time for IDs to work
                    self.components.insert(ctx.component.id, ctx.component_def.instantiate());
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
        let component = self.components.get(&connector.component)
            .ok_or(DefinitionError::InvalidConnector(connector))?;
        if let Some(_) = get_transparent(component) {
            let rerouted_def = self.rerouted_defs.get(&connector.component).unwrap();
            let pin_mapping = rerouted_def.pin_mapping.as_ref().unwrap();
            let input = pin_mapping.input.iter();
            let output = pin_mapping.output.iter();
            let mut pins = input.chain(output);

            // TODO: Do actual error handling
            let connectors = pins.nth(connector.pin as usize).unwrap();
            for connector in connectors {
                self.reroute_to_concrete_impl(*connector, rerouted_connectors)?;
            }

            return Ok(());
        }

        // Add concrete component
        rerouted_connectors.push(connector);
        Ok(())
    }

    fn reroute_connection(&self, connection: &Connection, registry: &Registry) -> Result<Vec<Connection>, DefinitionError> {
        let from = self.reroute_to_concrete(connection.from)?;
        let to: Vec<Connector> = connection.to.iter().map(|x| {
            self.reroute_to_concrete(*x)
        }).collect::<Result<Vec<Vec<Connector>>, _>>()?.into_iter().flatten().collect();

        // Each 'from' connector needs to be connected to all the 'to' connectors
        let mut connections: Vec<Connection> = from.iter().map(|&from| {
            let to = to.clone();
            Connection { from, to }
        }).collect();

        // Wire Clock components back into itself so that events repeat
        connections.iter_mut()
            .filter(|x| {
                let def_id = self.definition_mapping[&x.from.component];
                let def = registry.get_definition(def_id).unwrap();

                def.id == component::definition::CLOCK_ID
            })
            .for_each(|x| {
                let to_self = Connector { component: x.from.component, pin: 1 };
                x.to.push(to_self);
            });
        
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
pub struct Context<'a> {
    pub component: component::definition::Component,
    pub component_def: &'a ComponentDefinition,
    pub registry: &'a Registry,
}

#[derive(Debug, thiserror::Error)]
pub enum DefinitionError {
    #[error("Component with id {0} already exists, cannot take its place.")]
    ComponentIdAlreadyTaken(u32),

    #[error("Encountered a registry error.")]
    Registry(#[from] RegistryError),

    #[error("Invalid transparent component found. Context: {0}")]
    InvalidTransparentComponent(String),

    #[error("Invalid connector (0.component, 0.pin) found in circuit connections.")]
    InvalidConnector(Connector),
}


#[cfg(test)]
mod tests {
    use crate::{component::ComponentDefinition, Circuit};
    use super::{CircuitDefinition, Registry};

    // TODO: Check number of components inserted
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



