use std::collections::HashMap;
use crate::component::ComponentDefinition;

#[derive(Debug, Default)]
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

