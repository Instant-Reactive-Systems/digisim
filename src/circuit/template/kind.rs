#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize)]
pub enum ComponentKind {
    Composite,
    Optimized,
}

