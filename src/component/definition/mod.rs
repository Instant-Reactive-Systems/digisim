mod kind;
mod pins;
mod pin_mapping;
mod circuit;
mod component;
mod truth_table;
pub use kind::ComponentKind;
pub use pins::Pins;
pub use pin_mapping::PinMapping;
pub use circuit::Circuit;
pub use component::Component;
pub use truth_table::TruthTable;

use super::Component as ComponentTrait;
use derivative::Derivative;
use crate::circuit::Params;
use crate::circuit::registry::PREBUILT_REGISTRY;
use crate::component::Generic;

#[derive(Derivative, Debug, Clone, serde::Deserialize)]
#[derivative(PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    pub id: i32,
    pub name: String,
    #[serde(rename = "description")] 
    pub desc: String,
    #[serde(rename = "type")] 
    pub kind: ComponentKind,
    pub pins: Pins,
    pub pin_mapping: Option<PinMapping>,
    pub circuit: Option<Circuit>,
    pub truth_table: Option<TruthTable>,
    #[serde(rename = "booleanFunction")] 
    pub expr: Option<String>,

    #[serde(skip)]
    #[derivative(PartialEq = "ignore")]
    pub parsed_expr: Option<rustlogic::LogicNode>,
}

impl ComponentDefinition {
    /// Instantiates the component definition into a component.
    pub fn instantiate(&self, params: Params) -> Box<dyn ComponentTrait> {
        match self.kind {
            ComponentKind::Builtin => PREBUILT_REGISTRY.with(|reg| (reg.data[&self.id].factory)(params)),
            _ => Box::new(Generic { component_def: self }),
        }
    }

    /// Reroutes transparent component definition's IDs into the current circuit.
    pub fn reroute_component_def(&self, first_free_id: u32) -> Self {
        let mut new_component_def = self.clone();
        let circuit = new_component_def.circuit.as_mut().unwrap();

        // Update all components' IDs
        circuit.components.iter_mut().for_each(|x| x.id += first_free_id);
        
        // Update component IDs of all connections
        for connection in circuit.connections.iter_mut() {
            // Update 'from' connector and all 'to' connectors
            connection.from.component += first_free_id;
            connection.to.iter_mut().for_each(|x| x.component += first_free_id);
        }
        
        // Update input/output mapping
        let pin_mapping = new_component_def.pin_mapping.as_mut().unwrap();
        pin_mapping.input.iter_mut().flatten().for_each(|x| x.component += first_free_id);
        pin_mapping.output.iter_mut().flatten().for_each(|x| x.component += first_free_id);

        // Update params field
        if let Some(params) = circuit.params.as_mut() {
            *params = params.drain().map(|(id, params_)| (id + first_free_id, params_)).collect();
        }

        new_component_def
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Connection, Connector};

    #[test]
    fn and_gate() {
        let def = include_str!("../../../tests/assets/and_gate_definition.json");
        let parsed: ComponentDefinition = serde_json::from_str(def).unwrap();
        let result = ComponentDefinition {
            id: 1,
            name: "AndGate".into(),
            desc: "An AND gate component.".into(),
            kind: ComponentKind::Transparent,
            pins: Pins{
                input: vec!["A".into(), "B".into()],
                output: vec!["Y".into()],
            },
            pin_mapping: Some(PinMapping{
                input: vec![
                    vec![Connector { component: 0, pin: 0 }],
                    vec![Connector { component: 0, pin: 1 }],
                ],
                output: vec![
                    vec![
                        Connector { component: 1, pin: 2 },
                    ]
                ],
            }),
            circuit: Some(Circuit {
                params: None,
                components: vec![
                    Component { def_id: -1, id: 0 },
                    Component { def_id: -1, id: 1 },
                ],
                connections: vec![
                    Connection { 
                        from: Connector { component: 0, pin: 2 },
                        to: vec![
                            Connector { component: 1, pin: 0 },
                            Connector { component: 1, pin: 1 },
                        ],
                    },
                ],
            }),
            truth_table: Some(TruthTable { 
                inputs: vec![
                    vec![false, false],
                    vec![false, true],
                    vec![true, false],
                    vec![true, true],
                ],
                outputs: vec![
                    vec![false],
                    vec![false],
                    vec![false],
                    vec![true],
                ],
            }),
            expr: Some("A and B".into()),
            parsed_expr: None,
        };

        assert_eq!(parsed, result);
    }

    #[test]
    fn rerouting_works() {
        let def = ComponentDefinition {
            id: 1,
            name: "AndGate".into(),
            desc: "An AND gate component.".into(),
            kind: ComponentKind::Transparent,
            pins: Pins{
                input: vec!["A".into(), "B".into()],
                output: vec!["Y".into()],
            },
            pin_mapping: Some(PinMapping{
                input: vec![
                    vec![Connector { component: 0, pin: 0 }],
                    vec![Connector { component: 0, pin: 1 }],
                ],
                output: vec![
                    vec![
                        Connector { component: 1, pin: 2 },
                    ]
                ],
            }),
            circuit: Some(Circuit {
                params: None,
                components: vec![
                    Component { def_id: -1, id: 0 },
                    Component { def_id: -1, id: 1 },
                ],
                connections: vec![
                    Connection { 
                        from: Connector { component: 0, pin: 2 },
                        to: vec![
                            Connector { component: 1, pin: 0 },
                            Connector { component: 1, pin: 1 },
                        ],
                    },
                ],
            }),
            truth_table: None,
            expr: None,
            parsed_expr: None,
        };

        let new_def = def.reroute_component_def(5);

        let result = ComponentDefinition {
            id: 1,
            name: "AndGate".into(),
            desc: "An AND gate component.".into(),
            kind: ComponentKind::Transparent,
            pins: Pins{
                input: vec!["A".into(), "B".into()],
                output: vec!["Y".into()],
            },
            pin_mapping: Some(PinMapping{
                input: vec![
                    vec![Connector { component: 5, pin: 0 }],
                    vec![Connector { component: 5, pin: 1 }],
                ],
                output: vec![
                    vec![
                        Connector { component: 6, pin: 2 },
                    ]
                ],
            }),
            circuit: Some(Circuit {
                params: None,
                components: vec![
                    Component { def_id: -1, id: 5 },
                    Component { def_id: -1, id: 6 },
                ],
                connections: vec![
                    Connection { 
                        from: Connector { component: 5, pin: 2 },
                        to: vec![
                            Connector { component: 6, pin: 0 },
                            Connector { component: 6, pin: 1 },
                        ],
                    },
                ],
            }),
            truth_table: None,
            expr: None,
            parsed_expr: None,
        };
        
        assert_eq!(new_def, result);
    }
}

