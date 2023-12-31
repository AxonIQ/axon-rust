use fmodel_rust::aggregate::EventSourcedAggregate;
use fmodel_rust::decider::Decider;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use strum::IntoEnumIterator;
use synapse_client::apis::command_handlers_api::replace_command_handler;
use synapse_client::apis::configuration;
use synapse_client::models::{CommandHandlerRegistration, CommandMessage, CommandResponseMessage};

use crate::command::gift_card_command_handler::GiftCardState;
use crate::command::gift_card_event_repository::{AggregateError, AxonServerEventRepository};
use crate::gift_card_api::{GiftCardCommand, GiftCardEvent};

/// Pragmatic type alias for the EventSourcedAggregate
type GiftCardAggregate<'a> = EventSourcedAggregate<
    GiftCardCommand,
    GiftCardState,
    GiftCardEvent,
    AxonServerEventRepository,
    Decider<'a, GiftCardCommand, GiftCardState, GiftCardEvent>,
    i64,
    AggregateError,
>;

#[post("/commands", format = "application/json", data = "<command_message>")]
pub async fn commands(
    command_message: Json<CommandMessage>,
    app_state: &State<GiftCardAggregate<'_>>,
) -> Result<Json<CommandResponseMessage>, Status> {
    let command_message = command_message.into_inner();
    let command = command_message.to_gift_card_command();
    match command {
        Some(c) => {
            let result = app_state.handle(&c).await;
            match result {
                Ok(_) => Ok(Json(CommandResponseMessage {
                    id: command_message.to_owned().id,
                    meta_data: command_message.to_owned().meta_data,
                    payload: command_message.to_owned().payload,
                    payload_type: command_message.to_owned().payload_type,
                    payload_revision: match command_message.to_owned().payload_revision {
                        Some(revision) => Some(revision),
                        None => Some("0".to_string()),
                    },
                })),
                Err(_err) => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::InternalServerError),
    }
}

pub async fn register_gift_card_command_handler(
    configuration: &configuration::Configuration,
    context: &String,
    client_id: &String,
    component_name: &String,
    application_host: &String,
) {
    let registration = CommandHandlerRegistration {
        names: GiftCardCommand::iter().map(|v| v.payload_type()).collect(),
        endpoint: application_host.to_owned() + &*"/commands".to_string(),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(client_id.to_owned()),
        component_name: Some(component_name.to_owned()),
        load_factor: Some(100),
        concurrency: Some(8),
        enabled: Some(true),
        context: Some(context.to_owned()),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    replace_command_handler(
        configuration,
        context,
        "10fca0c4-3376-4ca2-a7c2-db2b75c250e0",
        Some(registration),
    )
    .await
    .unwrap();
}

/// Map to domain commands of type GiftCardCommand
trait ToGiftCardCommand {
    fn to_gift_card_command(&self) -> Option<GiftCardCommand>;
}

/// Map from Axon CommandMessage to domain events of type GiftCardCommand
impl ToGiftCardCommand for CommandMessage {
    fn to_gift_card_command(&self) -> Option<GiftCardCommand> {
        let value = self.payload.clone().unwrap().unwrap();
        let command = serde_json::from_value(value);
        match command {
            Ok(command) => Some(command),
            Err(_err) => None,
        }
    }
}
