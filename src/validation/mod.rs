mod requirements;
mod report;
pub use requirements::CombinationalRequirements;
pub use report::{ValidationReport, ValidationError, JsValidationReport, JsValidationError};

use crate::component::{Led, Switch};
use crate::sim::Event;
use crate::{Simulation, Circuit, wasm, log};
use crate::circuit::registry::{SWITCH_ID, LED_ID, REGISTRY};
use crate::component::definition::{ComponentDefinition, ComponentKind, Component};
use crate::circuit::{CircuitDefinition, Registry, Connection, Connector};
use ConversionError::*;
use rassert_rs::rassert;

#[wasm::wasm_bindgen(js_name = "test_combinational")]
pub fn js_test_combinational(component_def: wasm::JsValue, requirements: wasm::JsValue) -> JsValidationReport {
    let component_def = component_def.into_serde().expect("Expected the component definition to be in correct format.");
    let requirements = requirements.into_serde().expect("Expected the combinational requirements to be in correct format.");

    test_combinational(component_def, requirements).into()
}

pub fn test_combinational(component_def: ComponentDefinition, requirements: CombinationalRequirements) -> ValidationReport {
    let mut report = ValidationReport::default();
    
    // Validate test requirements
    if requirements.truth_table.inputs.is_empty() || requirements.truth_table.outputs.is_empty() {
        report.errors.push(ValidationError::EmptyTruthTable);
        return report;
    }

    // Validate component definition (capture all related errors and return if any of them failed
    // afterwards)
    let used = component_def.circuit.as_ref().unwrap().components.len() as u32;
    if !(used <= requirements.max_components.unwrap_or(u32::MAX)) {
        report.errors.push(ValidationError::MaxComponentsExceeded { used });
    }

    if !(component_def.pins.input.len() == requirements.truth_table.inputs[0].len()) {
        report.errors.push(ValidationError::InvalidComponentInterface { 
            is_input: true, 
            expected: requirements.truth_table.inputs[0].len() as u32, 
            actual: component_def.pins.input.len() as u32,
        });
    } else if !(component_def.pins.output.len() == requirements.truth_table.outputs[0].len()) {
        report.errors.push(ValidationError::InvalidComponentInterface { 
            is_input: false, 
            expected: requirements.truth_table.outputs[0].len() as u32, 
            actual: component_def.pins.output.len() as u32,
        });
    }

    // If any of the component definition validation failed, early exit
    if report.failure() {
        return report;
    }

    // Construct the temporary registry
    let mut temp_registry = Registry::default();
    REGISTRY.with(|reg| temp_registry = reg.lock().clone());

    // Construct the test circuit definition
    let circuit_def = to_test_circuit_definition(&mut temp_registry, component_def.clone()).unwrap();
    let circuit = Circuit::from_definition(&temp_registry, circuit_def).unwrap();

    // Construct the simulation
    let mut ctx = Simulation::default();
    ctx.circuit = circuit;

    for (inputs, expected_outputs) in requirements.truth_table.iter() {
        // Set inputs
        for (i, &input) in inputs.iter().enumerate() {
            let id = (i + 1) as u32;
            let switch = ctx.circuit.components.get_mut(&id).unwrap().as_any_mut().downcast_mut::<Switch>().unwrap();
            switch.output = input;
        }

        ctx.init();

        // Advance the simulation
        if let Some(max_runtime) = requirements.max_runtime {
            ctx.tick_for(max_runtime as usize);
        } else {
            unimplemented!()
        }

        // Get outputs
        let mut actual_outputs = Vec::with_capacity(expected_outputs.len());
        for (i, _) in expected_outputs.iter().enumerate() {
            let id = (i + inputs.len() + 1) as u32;
            let led = ctx.circuit.components.get(&id).unwrap().as_any().downcast_ref::<Led>().unwrap();
            actual_outputs.push(led.value);
        }

        // Process result
        if expected_outputs != &actual_outputs {
            report.errors.push(ValidationError::IncorrectOutputs {
                input: inputs.clone(),
                expected: expected_outputs.clone(),
                actual: actual_outputs,
            });
        }

        // Reset simulation state
        ctx.reset();
    }

    report
}

/// Convert a Transparent component definition into a test circuit definition.
fn to_test_circuit_definition(registry: &mut Registry, mut component_def: ComponentDefinition) -> Result<CircuitDefinition, ConversionError> {
    rassert!(component_def.kind == ComponentKind::Transparent, IncorrectKind);
    component_def.id = i32::MIN; // Reserved for temporary definitions

    // Insert the component definition into the temporary registry
    registry.insert(component_def.clone());

    let mut circuit_def = CircuitDefinition::default();
    circuit_def.id = component_def.id;
    circuit_def.name = component_def.name.clone();
    circuit_def.desc = component_def.desc.clone();
    circuit_def.components.push(Component {
        id: 0,
        def_id: component_def.id,
    });

    // Create and connect the source components
    let num_inputs = component_def.pins.input.len();
    for i in 0..num_inputs {
        let component_pin = i as u32;
        let id = component_pin + 1;
        circuit_def.components.push(Component { id, def_id: SWITCH_ID });
        circuit_def.connections.push(Connection {
            from: Connector::new(id, 0),
            to: vec![Connector::new(0, component_pin)],
        });
    }

    // Create and connect the output components
    let num_outputs = component_def.pins.output.len();
    for i in 0..num_outputs {
        let component_pin = (i + num_inputs) as u32;
        let id = (i + num_inputs + 1) as u32;
        circuit_def.components.push(Component { id, def_id: LED_ID });
        circuit_def.connections.push(Connection {
            from: Connector::new(0, component_pin),
            to: vec![Connector::new(id, 0)],
        });
    }

    Ok(circuit_def)
}

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("Cannot convert non-transparent component definition to a circuit definition.")]
    IncorrectKind,
}

