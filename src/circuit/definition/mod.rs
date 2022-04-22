use std::collections::HashMap;

use crate::component::definition::Component;
use super::{Connection, Params, Id};

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CircuitDefinition {
    pub id: i32,
    pub name: String,
    #[serde(rename = "description")] 
    pub desc: String,
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
    pub params: Option<HashMap<Id, Params>>,
}

