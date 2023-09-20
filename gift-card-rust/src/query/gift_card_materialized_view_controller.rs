use fmodel_rust::materialized_view::{MaterializedView, ViewStateRepository};
use fmodel_rust::view::View;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use strum::IntoEnumIterator;
use synapse_client::apis::configuration;
use synapse_client::apis::event_handlers_api::replace_event_handler;
use synapse_client::apis::query_handlers_api::replace_query_handler;
use synapse_client::models::{
    EventHandlerRegistration, EventMessage, QueryHandlerRegistration, QueryMessage,
    QueryResponseMessage,
};

use crate::command::gift_card_event_repository::ToGiftCardEvent;
use crate::gift_card_api::{GiftCardEvent, GiftCardIssued, GiftCardQuery};
use crate::query::gift_card_event_handler::GiftCardViewState;
use crate::query::gift_card_view_state_repository::{
    InMemoryViewStateRepository, MaterializedViewError,
};

/// Pragmatic type alias for the MaterializedView
type GiftCardMaterializedView<'a> = MaterializedView<
    GiftCardViewState,
    GiftCardEvent,
    InMemoryViewStateRepository,
    View<'a, GiftCardViewState, GiftCardEvent>,
    MaterializedViewError,
>;

#[post("/events", format = "application/json", data = "<event_message>")]
pub async fn events(
    event_message: Json<EventMessage>,
    materialized_view: &State<GiftCardMaterializedView<'_>>,
) -> Result<Json<GiftCardViewState>, Status> {
    let event = event_message.into_inner();
    let event = event.to_gift_card_event().unwrap();
    materialized_view
        .handle(&event)
        .await
        .map_err(|_err| Status::InternalServerError)
        .map(Json)
}

#[post("/queries", format = "application/json", data = "<query_message>")]
pub async fn queries(
    query_message: Json<QueryMessage>,
    repository: &State<InMemoryViewStateRepository>,
) -> Result<Json<QueryResponseMessage>, Status> {
    let query = query_message.into_inner();
    let query = query.to_gift_card_query().unwrap();
    match query {
        GiftCardQuery::ById(q) => {
            let state = repository
                .fetch_state(&GiftCardEvent::Issue(GiftCardIssued {
                    id: q.id.to_owned(),
                    amount: 0,
                }))
                .await
                .unwrap();
            match state {
                Some(s) => Ok(Json(QueryResponseMessage {
                    id: None,
                    meta_data: None,
                    payload: Some(Some(serde_json::to_value(s).unwrap())),
                    payload_type: Option::from("GiftCardViewState".to_string()),
                    payload_revision: None,
                })),
                None => Err(Status::NotFound),
            }
        }
        GiftCardQuery::All(..) => {
            let states = repository.states.lock().unwrap();
            let states: Vec<GiftCardViewState> = states.values().cloned().collect();
            Ok(Json(QueryResponseMessage {
                id: None,
                meta_data: None,
                payload: Some(Some(serde_json::to_value(states).unwrap())),
                payload_type: Option::from("GiftCardViewState".to_string()),
                payload_revision: None,
            }))
        }
    }
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

pub async fn register_gift_card_query_handler(
    configuration: &configuration::Configuration,
    context: &String,
    client_id: &String,
    component_name: &String,
    application_host: &String,
) {
    let registration = QueryHandlerRegistration {
        names: GiftCardQuery::iter().map(|v| v.payload_type()).collect(),
        endpoint: application_host.to_owned() + &*"/queries".to_string(),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(client_id.to_owned()),
        component_name: Some(component_name.to_owned()),
        enabled: Some(true),
        context: Some(context.to_owned()),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    replace_query_handler(
        configuration,
        context,
        "10fca0c4-3376-4ca2-a7c2-db2b75c250e2",
        Some(registration),
    )
    .await
    .unwrap();
}

/// Map to domain queries of type GiftCardQuery
trait ToGiftCardQuery {
    fn to_gift_card_query(&self) -> Option<GiftCardQuery>;
}

/// Map from Axon QueryMessage to domain events of type GiftCardQuery
impl ToGiftCardQuery for QueryMessage {
    fn to_gift_card_query(&self) -> Option<GiftCardQuery> {
        let value = self.payload.clone().unwrap().unwrap();
        let query = serde_json::from_value(value);
        match query {
            Ok(query) => Some(query),
            Err(_err) => None,
        }
    }
}
