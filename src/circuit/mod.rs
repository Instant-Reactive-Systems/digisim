mod id;
mod connector;
mod connection;
mod state;
mod registry;
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
use crate::component::{self, Component, ComponentDefinition};
use registry::RegistryError;

/// A self-contained collection of all components and its wiring.
#[derive(Debug, Default)]
pub struct Circuit {
    pub components: HashMap<Id, Box<dyn Component>>,
    pub output_components: Vec<Id>,
    pub connections: HashMap<Connector, Vec<Connector>>,
}

impl Circuit {
    pub fn from_definition(registry: &Registry, circuit_def: CircuitDefinition) -> Result<Self, DefinitionError> {
        let mut circuit = Circuit::default();

        // 1.) Iterate through the components in the circuit
        // 2.) Process only non-transparent components, and put transparent ones into a separate
        // list
        // 3.) Then iterate through transparent components and recursively process inner transparent 
        // components by remapping their connections down to their concrete components
        // Note: Do not add inner transparent components to the components field of the Circuit
        
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
                ComponentKind::Transparent => transparent_components.push(ctx),
            }
        }

        for transparent in transparent_components {
            circuit.process_transparent(transparent)?;
        }

        todo!()
    }

    fn process_builtin(&mut self, ctx: Context) -> Result<(), DefinitionError> {
        rassert!(!self.components.contains_key(&ctx.component.id), DefinitionError::ComponentIdAlreadyTaken(ctx.component.id));

        self.components.insert(ctx.component.id, ctx.component_def.instantiate());
        self.output_components.push(ctx.component.id);

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
        let rerouted_def = ctx.component_def.reroute_component_def(self.components.len() as u32);
        let circuit = rerouted_def.circuit.as_ref().ok_or(DefinitionError::InvalidTransparentComponent("No circuit field".into()))?;
        
        for &component in circuit.components.iter() {
            let component_def = ctx.registry.get_definition(component.def_id)?;
            let ctx = Context {
                component: ctx.component,
                component_def,
                registry: ctx.registry,
            };

            match component_def.kind {
                ComponentKind::Builtin => self.process_builtin(ctx)?,
                ComponentKind::Compiled => self.process_compiled(ctx)?,
                ComponentKind::Functional => self.process_functional(ctx)?,
                ComponentKind::Transparent => transparent_components.push(ctx),
            }
        }

        for transparent in transparent_components {
            self.process_transparent(transparent)?;
        }

        Ok(())
    }    
}

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
}


#[cfg(test)]
mod tests {
    
}



