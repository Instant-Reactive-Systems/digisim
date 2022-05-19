use std::any::Any;
use rassert_rs::rassert;

use crate::circuit::{Connector, Params};
use super::Component;
use crate::sim::{Event, UserEvent, UserEventError};
use UserEventError::*;
use pitch_detection::detector::{PitchDetector, mcleod::McLeodDetector};

#[derive(Debug, Clone, Default)]
pub struct AudioListener {
	pub(crate) output: bool,

	delay: u32,
    sample_rate: u64,
    size: u64,
    padding: u64,
    power_threshold: f64,
    clarity_threshold: f64,
    freq_threshold: f64,
}

impl Component for AudioListener {
	fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
		Some(vec![(0, self.output)])
	}

	fn update(&mut self, event: Event) {
        match event.src.pin {
			0 => self.output = event.value,
			_ => {}
		}
    }

	fn set_pin(&mut self, _pin: u32, _event: Event) {
        // set_pin is not implemented for source components
        unreachable!()
	}

	fn get_state(&self) -> serde_json::Value {
        unimplemented!("Switch does not implement get_state since it is not an output component.");
	}

	fn delay(&self) -> u32 {
		self.delay
	}

	fn is_source(&self) -> bool {
		true
	}
    
    fn is_output(&self) -> bool {
        false
    }

	fn as_any(&self) -> &dyn Any {
		self
	}

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

	fn process_user_event(&self, user_event: UserEvent) -> Result<Vec<Event>, UserEventError> {
        let signal: Vec<f64> = serde_json::from_value(user_event.payload)
            .or(Err(InvalidPayload("AudioListener receives a Vec<f64> buffer as input.".into())))?;

        let mut detector = McLeodDetector::new(self.size as usize, self.padding as usize);
        let pitch = detector
            .get_pitch(&signal, self.sample_rate as usize, self.power_threshold, self.clarity_threshold)
            .unwrap();

        // Only emit a valid signal when the pitch is over the threshold
        if pitch.frequency < self.freq_threshold {
            Ok(vec![])
        } else {
            let src = Connector { component: user_event.component_id, pin: 0 };
            Ok(vec![Event::new(true, src)])
        }
	}

    fn reset(&mut self) {
        self.output = false;
    }
}

impl AudioListener {
    pub fn from_params(params: Params) -> Self {
        let delay = if let Some(param) = params.get("delay") {
            param.as_u64().unwrap() as u32
        } else {
            1
        };

        // Needed by audio-detection crate
        let sample_rate = params.get("sampleRate").expect("Expected 'sampleRate' property in params.")
            .as_u64().expect("Expected the 'sampleRate' param to be a u64.");
        let size = params.get("size").expect("Expected 'size' property in params.")
            .as_u64().expect("Expected the 'size' param to be a u64.");
        let padding = params.get("padding").expect("Expected 'padding' property in params.")
            .as_u64().expect("Expected the 'padding' param to be a u64.");
        let power_threshold = params.get("powerThreshold").expect("Expected 'powerThreshold' property in params.")
            .as_f64().expect("Expected the 'powerThreshold' param to be a f64.");
        let clarity_threshold = params.get("clarityThreshold").expect("Expected 'clarityThreshold' property in params.")
            .as_f64().expect("Expected the 'clarityThreshold' param to be a f64.");

        let freq_threshold = params.get("freqThreshold").expect("Expected 'freqThreshold' property in params.")
            .as_f64().expect("Expected the 'freqThreshold' param to be a f64.");

        Self {
            delay,
            sample_rate,
            size,
            padding,
            power_threshold,
            clarity_threshold,
            freq_threshold,
            ..Default::default()
        }
    }
}

