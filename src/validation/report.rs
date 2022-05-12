#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorKind {
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ValidationError {
    IncorrectOutputs {
        input: Vec<bool>,
        expected: Vec<bool>,
        actual: Vec<bool>,
    },
    MaxComponentsExceeded {
        used: u32,
        max_allowed: u32,
    },
    InvalidComponentInterface {
        kind: ConnectorKind,
        expected: u32,
        actual: u32,
    },
}

#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ValidationReport {
    pub errors: Vec<ValidationError>,
}

impl ValidationReport {
    pub fn merge(&mut self, other: ValidationReport) {
        self.errors.extend_from_slice(&other.errors);
    }

    pub fn success(&self) -> bool {
        self.errors.is_empty()
    }
    
    pub fn failure(&self) -> bool {
        !self.success()
    }
}

