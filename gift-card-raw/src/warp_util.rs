use serde_derive::Serialize;
use synapse_client::models::{CommandMessage, CommandResponseMessage};
use warp::http::StatusCode;
use warp::reply;
use warp::reply::{Json, WithStatus};

#[derive(Serialize)]
pub struct Empty {}

#[derive(Serialize)]
pub struct HandlerErrorMessage {
    pub code: u16,
    pub message: String,
}

pub enum HandlerResult {
    Error(HandlerErrorMessage),
    CommandSuccess(CommandMessage),
    EventSuccess,
}

impl HandlerResult {
    pub fn into_json(self) -> WithStatus<Json> {
        match self {
            HandlerResult::Error(hem) => {
                let rep = reply::json(&hem);
                reply::with_status(rep, StatusCode::from_u16(hem.code).unwrap())
            }
            HandlerResult::CommandSuccess(command_message) => {
                let response_message = CommandResponseMessage {
                    id: command_message.id,
                    meta_data: command_message.meta_data,
                    payload: command_message.payload,
                    payload_type: command_message.payload_type,
                    payload_revision: command_message.payload_revision,
                };
                let reply = reply::json(&response_message);
                reply::with_status(reply, StatusCode::OK)
            }
            HandlerResult::EventSuccess => {
                let cr = Empty {};
                let rep = reply::json(&cr);
                reply::with_status(rep, StatusCode::OK)
            }
        }
    }
}
