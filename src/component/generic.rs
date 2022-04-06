use super::{Component, ComponentDefinition};
use crate::circuit::Id;

pub struct Generic<'a> {
    pub component_def: &'a ComponentDefinition,
}

