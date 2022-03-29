use std::collections::{HashMap, HashSet};
use crate::{Component, Connection, Slot, rassert};
use crate::component::{Transparent, Source};
use std::any::Any;

/// A self-contained collection of all components and its wiring.
#[derive(Debug, Default)]
pub struct Circuit {
    pub components: HashMap<ComponentId, Box<dyn Component>>,
    pub connections: HashMap<Connector, Vec<Connector>>,
}

pub type ComponentId = u32;

