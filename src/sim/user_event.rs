use crate::circuit::Id;

#[derive(Debug, thiserror::Error)]
pub enum UserEventError {
	#[error("Invalid payload received. Context: {0}")]
	InvalidPayload(String),
}

#[derive(serde::Deserialize)]
pub struct UserEvent {
	pub component_id: Id,
	pub payload: String,
}
