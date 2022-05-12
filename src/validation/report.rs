#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorKind {
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ValidationErrorKind {
    IncorrectOutputs,
    MaxComponentsExceeded,
    InvalidComponentInterface,
    EmptyTruthTable,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ValidationErrorData {
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
    EmptyTruthTable,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ValidationError {
    kind: ValidationErrorKind,
    data: ValidationErrorData,
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

impl Into<ValidationError> for ValidationErrorData {
    fn into(self) -> ValidationError {
        match self {
            Self::IncorrectOutputs { .. } => ValidationError {
                kind: ValidationErrorKind::IncorrectOutputs,
                data: self,
            },
            Self::MaxComponentsExceeded { .. } => ValidationError {
                kind: ValidationErrorKind::MaxComponentsExceeded,
                data: self,
            },
            Self::InvalidComponentInterface { .. } => ValidationError {
                kind: ValidationErrorKind::InvalidComponentInterface,
                data: self,
            },
            Self::EmptyTruthTable { .. } => ValidationError {
                kind: ValidationErrorKind::EmptyTruthTable,
                data: self,
            },
        }
    }
}

