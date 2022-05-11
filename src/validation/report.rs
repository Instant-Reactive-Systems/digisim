use crate::wasm;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorKind {
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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
    EmptyTruthTable,
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



#[wasm::wasm_bindgen(js_class = ValidationReport, getter_with_clone)]
pub struct JsValidationReport {
    pub errors: wasm::Array,
}

impl Into<JsValidationReport> for ValidationReport {
    fn into(self) -> JsValidationReport {
        JsValidationReport {
            errors: wasm::JsValue::from_serde(&self.errors).unwrap().into(),
        }
    }
}

