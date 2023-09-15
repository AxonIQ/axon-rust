use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use prost::Message;
use serde_json::Value;

pub mod commands;
pub mod events;
pub mod queries;

pub trait AxonMessage {
    fn name() -> &'static str;
}

pub fn value_to_message<T>(value: Value) -> T
where
    T: Message + Default,
{
    let bytes = match value {
        Value::String(encoded) => match general_purpose::STANDARD.decode(encoded) {
            Ok(b) => Bytes::from(b),
            Err(e) => panic!("Error decoding command. {}", e),
        },
        _ => panic!("Expected value to be a string but was: {}", value),
    };
    Message::decode(bytes).unwrap()
}

pub fn message_to_payload<T>(message: &T) -> Option<Option<Value>>
where
    T: Message,
{
    let bytes = message.encode_to_vec();
    let encoded = general_purpose::STANDARD.encode(bytes);
    Some(Some(Value::String(encoded)))
}
