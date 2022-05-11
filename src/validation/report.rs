use crate::wasm;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum ValidationError {
    IncorrectOutputs {
        input: Vec<bool>,
        expected: Vec<bool>,
        actual: Vec<bool>,
    },
    MaxComponentsExceeded {
        used: u32,
    },
    InvalidComponentInterface {
        is_input: bool,
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



#[wasm::wasm_bindgen(js_class = ValidationError, getter_with_clone)]
pub struct JsValidationError {
    pub kind: wasm::JsString,
    pub data: wasm::JsValue,
}

#[wasm::wasm_bindgen(js_class = ValidationReport, getter_with_clone)]
pub struct JsValidationReport {
    pub errors: wasm::Array,
}

#[wasm::wasm_bindgen]
impl JsValidationReport {
    pub fn success(&self) -> bool {
        self.errors.length() == 0
    }

    pub fn failed(&self) -> bool {
        !self.success()
    }
}

impl Into<JsValidationError> for ValidationError {
    fn into(self) -> JsValidationError {
        use ValidationError::*;

        match self {
            IncorrectOutputs { input, expected, actual } => JsValidationError {
                kind: "incorrect-outputs".into(),
                data: wasm::JsValue::from_serde(&serde_json::json!({
                    "input": input,
                    "expected": expected,
                    "actual": actual,
                })).unwrap(),
            },
            MaxComponentsExceeded { used } => JsValidationError {
                kind: "max-components-exceeded".into(),
                data: wasm::JsValue::from_serde(&serde_json::json!({
                    "used": used,
                })).unwrap(),
            },
            InvalidComponentInterface { is_input, expected, actual } => JsValidationError {
                kind: "invalid-component-inputs".into(),
                data: wasm::JsValue::from_serde(&serde_json::json!({
                    "is_input": is_input,
                    "expected": expected,
                    "actual": actual,
                })).unwrap(),
            },
            EmptyTruthTable => JsValidationError {
                kind: "empty-truth-table".into(),
                data: wasm::JsValue::UNDEFINED,
            }
        }
    }
}

impl Into<JsValidationReport> for ValidationReport {
    fn into(self) -> JsValidationReport {
        JsValidationReport {
            errors: wasm::JsValue::from_serde(&self).unwrap().into(),
        }
    }
}

