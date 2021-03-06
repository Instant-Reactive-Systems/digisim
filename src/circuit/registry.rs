use std::collections::HashMap;
use crate::{component::*, wasm};
use crate::component::definition::{Pins, ComponentKind};
use parking_lot::Mutex;

use super::Params;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Registry {
    components: HashMap<i32, ComponentDefinition>,
}

impl Registry {
    pub fn replace(&mut self, other: Registry) {
        self.components = other.components;
    }

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
    pub factory: Box<dyn Fn(Params) -> Box<dyn Component>>,
}

thread_local! {
    pub static PREBUILT_REGISTRY: PrebuiltRegistry = PrebuiltRegistry::default();
    pub static REGISTRY: Mutex<Registry> = Mutex::new(Registry::default());
}

#[wasm::wasm_bindgen]
pub fn set_registry(registry: wasm::JsValue) {
    let registry = registry.into_serde().expect("Expected the registry to be in correct format.");
    REGISTRY.with(|reg| reg.lock().replace(registry));
}

#[wasm::wasm_bindgen]
pub fn update_registry(definition: wasm::JsValue) {
    let component_def = definition.into_serde().expect("Expected the component definition to be in correct format");
    REGISTRY.with(|reg| reg.lock().insert(component_def))
}

// Prebuilt IDs
pub const NAND_ID: i32 = -1;
pub const TRISTATE_ID: i32 = -2;
pub const CLOCK_ID: i32 = -3;
pub const GROUND_ID: i32 = -4;
pub const SOURCE_ID: i32 = -5;
pub const SWITCH_ID: i32 = -6;
pub const LED_ID: i32 = -7;

impl Default for PrebuiltRegistry {
    fn default() -> Self {
        let mut data = HashMap::new();

        // ===== Populate prebuilt registry
        // Nand
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
            factory: Box::new(|params| Box::new(Nand::from_params(params))),
        });

        // Tristate
        data.insert(-2, PrebuiltEntry {
            def: ComponentDefinition {
                id: -2,
                name: "Tristate".into(),
                desc: "Tristate component with the capability of not propagating signals.".into(),
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
            factory: Box::new(|params| Box::new(Tristate::from_params(params))),
        });

        // Clock
        data.insert(-3, PrebuiltEntry {
            def: ComponentDefinition {
                id: -3,
                name: "Clock".into(),
                desc: "Emits signals repeatingly.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec![],
                    output: vec!["Y".into()],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|params| Box::new(Clock::from_params(params))),
        });

        // Ground
        data.insert(-4, PrebuiltEntry {
            def: ComponentDefinition {
                id: -4,
                name: "Ground".into(),
                desc: "Emits a constant 0.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec![],
                    output: vec!["Y".into()],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|params| Box::new(Ground::from_params(params))),
        });

        // Source
        data.insert(-5, PrebuiltEntry {
            def: ComponentDefinition {
                id: -5,
                name: "Source".into(),
                desc: "Emits a constant 1.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec![],
                    output: vec!["Y".into()],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|params| Box::new(Source::from_params(params))),
        });

        // Switch
        data.insert(-6, PrebuiltEntry {
            def: ComponentDefinition {
                id: -6,
                name: "Switch".into(),
                desc: "User-input component which emits whichever state it is currently on.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec![],
                    output: vec!["Y".into()],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|params| Box::new(Switch::from_params(params))),
        });

        // Led
        data.insert(-7, PrebuiltEntry {
            def: ComponentDefinition {
                id: -7,
                name: "Led".into(),
                desc: "Output component which only has a single state.".into(),
                kind: ComponentKind::Builtin,
                pins: Pins {
                    input: vec!["Y".into()],
                    output: vec![],
                },
                pin_mapping: None,
                circuit: None,
                truth_table: None,
                expr: None,
                parsed_expr: None,
            },
            factory: Box::new(|params| Box::new(Led::from_params(params))),
        });

        Self {
            data,
        }
    }
}

