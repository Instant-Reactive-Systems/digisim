use std::any::Any;
use rassert_rs::rassert;

use crate::circuit::{Connector, Params};
use super::Component;
use crate::sim::{Event, UserEvent, UserEventError};
use UserEventError::*;

#[derive(Debug, Clone, Default)]
pub struct Switch {
	output: bool,

	delay: u32,
}

impl Component for Switch {
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

	fn process_user_event(&self, user_event: UserEvent) -> Result<Vec<Event>, UserEventError> {
        rassert!(user_event.payload.is_string() && user_event.payload.as_str().unwrap() == "toggle", 
                 InvalidPayload("Switch only receives the message 'toggle'.".into()));

		let src = Connector { component: user_event.component_id, pin: 0 };
        Ok(vec![Event::new(!self.output, src)])
	}
}

impl Switch {
    pub fn from_params(params: Params) -> Self {
        let delay = if let Some(param) = params.get("delay") {
            param.as_u64().unwrap() as u32
        } else {
            1
        };

        Self {
            delay,
            ..Default::default()
        }
    }
}

