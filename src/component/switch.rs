use std::any::Any;
use crate::circuit::Connector;
use super::Component;
use crate::sim::{Event, UserEvent, UserEventError};

#[derive(Debug, Clone, Default)]
pub struct Switch {
	output: bool,

	delay: u32,
}

impl Component for Switch {
	fn evaluate(&self) -> Option<Vec<(u32, bool)>> {
		Some(vec![(0, self.output)])
	}

	fn update(&mut self, _event: Event) {}

	fn set_pin(&mut self, pin: u32, event: Event) {
		match pin {
			0 => self.output = event.value,
			_ => {}
		}
	}

	fn get_state(&self) -> serde_json::Value {
		todo!()
	}

	fn delay(&self) -> u32 {
		self.delay
	}

	fn is_source(&self) -> bool {
		false
	}

	fn as_any(&self) -> &dyn Any {
		self
	}

	fn process_user_event(&self, user_event: UserEvent) -> Result<Vec<Event>, UserEventError> {
		let src = Connector { component: user_event.component_id, pin: 0 };
		match user_event.payload.as_ref() {
			"toggle" => Ok(vec![Event::new(!self.output, src)]),
			_ => Err(UserEventError::InvalidPayload("Switch only receives the message 'toggle'.".into())),
		}
	}
}

