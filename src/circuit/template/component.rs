

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Component {
    #[serde(rename = "definitionId")] pub template_id: i32,
    pub id: u32,
}

