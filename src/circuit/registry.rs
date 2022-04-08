use std::collections::HashMap;
use crate::component::{self, ComponentDefinition, Component, definition::{Pins, ComponentKind}, Nand};

#[derive(Debug)]
pub struct Registry {
    components: HashMap<i32, ComponentDefinition>,
}

impl Registry {
    pub fn insert(&mut self, def: ComponentDefinition) {
        self.components.insert(def.id, def);
    }

    pub fn get_definition(&self, id: i32) -> Result<&ComponentDefinition, RegistryError> {
        self.components.get(&id).ok_or(RegistryError::InvalidDefinitionId(id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("Invalid definition id {0} received.")]
    InvalidDefinitionId(i32),
}

impl Default for Registry {
    fn default() -> Self {
        let mut components = HashMap::new();
        PREBUILT_REGISTRY.with(|reg| {
            for prebuilt in reg.data.values() {
                components.insert(prebuilt.def.id, prebuilt.def.clone());
            }
        });
        
        Self {
            components,
        }
    }
}

pub struct PrebuiltRegistry {
    pub data: HashMap<i32, PrebuiltEntry>,
}

pub struct PrebuiltEntry {
    pub def: ComponentDefinition,
    pub factory: Box<dyn Fn() -> Box<dyn Component>>,
}

thread_local! {
    pub static PREBUILT_REGISTRY: PrebuiltRegistry = PrebuiltRegistry::default();
}

impl Default for PrebuiltRegistry {
    fn default() -> Self {
        let mut data = HashMap::new();

        data.insert(-1, PrebuiltEntry {
            def: ComponentDefinition {
                id: -1,
                name: "NAND Gate".into(),
                desc: "It's a NAND gate.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec!["A".into(), "B".into()],
                    output: vec!["Y".into()],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|| Box::new(Nand::default())),
        });

        Self {
            data,
        }
    }
}

