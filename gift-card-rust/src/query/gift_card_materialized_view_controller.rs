use fmodel_rust::materialized_view::MaterializedView;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use strum::IntoEnumIterator;
use synapse_client::apis::configuration;
use synapse_client::apis::event_handlers_api::replace_event_handler;
use synapse_client::models::{EventHandlerRegistration, EventMessage};

use crate::gift_card_api::GiftCardEvent;
use crate::query::gift_card_event_handler::GiftCardViewState;
use crate::query::gift_card_view_state_repository::{
    InMemoryViewStateRepository, MaterializedViewError,
};

#[post("/events", format = "application/json", data = "<event_message>")]
pub async fn events(
    event_message: Json<EventMessage>,
    materialized_view: &State<
        MaterializedView<
            '_,
            GiftCardViewState,
            GiftCardEvent,
            InMemoryViewStateRepository,
            MaterializedViewError,
        >,
    >,
) -> Result<Json<GiftCardViewState>, Status> {
    let event = event_message.into_inner();
    let event = event.to_gift_card_event().unwrap();
    materialized_view
        .handle(&event)
        .await
        .map_err(|_err| Status::InternalServerError)
        .map(Json)
}

pub async fn register_gift_card_event_handler(
    configuration: &configuration::Configuration,
    context: &String,
    client_id: &String,
    component_name: &String,
    application_host: &String,
) {
    let registration = EventHandlerRegistration {
        names: GiftCardEvent::iter().map(|v| v.payload_type()).collect(),
        endpoint: application_host.to_owned() + &*"/events".to_string(),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(client_id.to_owned()),
        component_name: Some(component_name.to_owned()),
        batch_size: None,
        start: None,
        enabled: Some(true),
        context: Some(context.to_owned()),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    replace_event_handler(
        configuration,
        context,
        "10fca0c4-3376-4ca2-a7c2-db2b75c250e1",
        Some(registration),
    )
    .await
    .unwrap();
}

/// Map to domain events of type GiftCardEvent
trait ToGiftCardEvent {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent>;
}

/// Map from Axon EventMessage to domain events of type GiftCardEvent
impl ToGiftCardEvent for EventMessage {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent> {
        let value = self.payload.clone().unwrap().unwrap();
        let event = serde_json::from_value(value);
        match event {
            Ok(event) => Some(event),
            Err(_err) => None,
        }
    }
}
