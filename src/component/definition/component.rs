

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, serde::Deserialize)]
pub struct Component {
    #[serde(rename = "definitionId")] pub def_id: i32,
    pub id: u32,
}

