use redis::Msg;
use std::{error::Error, fmt::Display};
use todel::models::ServerPayload;

/// An Error that represents a Payload not being found.
#[derive(Debug)]
pub struct PayloadNotFound;

impl Display for PayloadNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Payload Not Found")
    }
}

impl Error for PayloadNotFound {}

/// A function that simplifies deserializing a message Payload.
pub fn deserialize_message(payload: Msg) -> Result<ServerPayload, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_str::<ServerPayload>(
        &payload
            .get_payload::<String>()
            .map_err(|_| PayloadNotFound)?,
    )?)
}
