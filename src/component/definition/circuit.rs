use std::collections::HashMap;

use crate::circuit::{Connection, Id, Params};
use super::Component;

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct Circuit {
    pub components: Vec<Component>,
    pub connections: Vec<Connection>,
    pub params: Option<HashMap<Id, Params>>,
}
