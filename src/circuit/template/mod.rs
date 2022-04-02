mod kind;
mod circuit;
mod component;
mod connector;
mod connection;
mod pins;
pub use kind::ComponentKind;
pub use circuit::Circuit;
pub use component::Component;
pub use connector::Connector;
pub use connection::Connection;
pub use pins::Pins;

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: i32,
    pub name: String,
    #[serde(rename = "description")] pub desc: String,
    #[serde(rename = "type")] pub kind: ComponentKind,
    pub pins: Pins,
    pub circuit: Option<Circuit>,
    pub truth_table: Option<Vec<Vec<bool>>>,
    #[serde(rename = "booleanFunction")] pub expr: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_gate() {
        let def = include_str!("../../../tests/assets/and_gate_definition.json");
        let template: Template = serde_json::from_str(def).unwrap();
        let result = Template {
            id: 1,
            name: "AndGate".into(),
            desc: "An AND gate component.".into(),
            kind: ComponentKind::Composite,
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
                    Component { template_id: -1, id: 0 },
                    Component { template_id: -1, id: 1 },
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

        assert_eq!(template, result);
    }
}

