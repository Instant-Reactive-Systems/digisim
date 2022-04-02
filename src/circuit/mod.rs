mod id;
mod connector;
mod state;
mod registry;
mod template;
pub use id::Id;
pub use connector::Connector;
pub use state::CircuitState;
pub use registry::Registry;
pub use template::Template;

use std::collections::HashMap;
use crate::Component;

/// A self-contained collection of all components and its wiring.
#[derive(Debug, Default)]
pub struct Circuit {
    pub components: HashMap<Id, Box<dyn Component>>,
    pub connections: HashMap<Connector, Vec<Connector>>,
}

