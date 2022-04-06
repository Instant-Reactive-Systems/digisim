use crate::component::definition::Component;
use super::Connection;

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircuitDefinition {
    pub id: i32,
    pub name: String,
    #[serde(rename = "description")] pub desc: String,
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
}

