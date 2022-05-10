#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize)]
pub enum ComponentKind {
    Builtin,
    Transparent,
    Compiled,
    Functional,
}

