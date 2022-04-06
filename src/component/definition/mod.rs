mod kind;
mod pins;
mod circuit;
mod component;
pub use kind::ComponentKind;
pub use pins::Pins;
pub use circuit::Circuit;
pub use component::Component;

use super::Component as ComponentTrait;

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    pub id: i32,
    pub name: String,
    #[serde(rename = "description")] pub desc: String,
    #[serde(rename = "type")] pub kind: ComponentKind,
    pub pins: Pins,
    pub circuit: Option<Circuit>,
    pub truth_table: Option<Vec<Vec<bool>>>,
    #[serde(rename = "booleanFunction")] pub expr: Option<String>,
}

impl ComponentDefinition {
    pub fn instantiate(&self) -> Box<dyn ComponentTrait> {
        match self.kind {
            ComponentKind::Builtin => {
                unimplemented!()
            },
            ComponentKind::Transparent => {
                unimplemented!()
            },
            ComponentKind::Compiled => {
                unimplemented!()
            },
            ComponentKind::Functional => {
                unimplemented!()
            },
        }
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
                input: vec![
                    Connector { component: 0, pin: 0 },
                    Connector { component: 0, pin: 1 },
                ],
                output: vec![
                    Connector { component: 1, pin: 2 },
                ],
            },
            circuit: Some(Circuit {
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
            truth_table: Some(vec![
                vec![false, false, false],
                vec![false,  true, false],
                vec![ true, false, false],
                vec![ true,  true,  true],
            ]),
            expr: Some("A and B".into()),
        };

        assert_eq!(parsed, result);
    }
}

