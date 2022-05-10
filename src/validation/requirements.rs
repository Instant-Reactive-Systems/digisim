use crate::component::definition::TruthTable;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CombinationalRequirements {
    pub max_runtime: Option<u32>,
    pub max_components: Option<u32>,
    pub truth_table: TruthTable,
}

