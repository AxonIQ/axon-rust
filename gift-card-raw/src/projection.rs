use crate::messages::events::{GiftCardCanceled, GiftCardIssued, GiftCardRedeemed};
use crate::messages::AxonMessage;
use crate::warp_util::HandlerResult;
use crate::CLIENT_ID;
use once_cell::sync::Lazy;
use synapse_client::apis::configuration::Configuration;
use synapse_client::apis::event_handlers_api::register_event_handler;
use synapse_client::models::{EventHandlerRegistration, EventMessage};
use warp::Filter;

static PROJECTION: Lazy<GiftCardProjection> = Lazy::new(GiftCardProjection::new);

pub struct GiftCardProjection {}

impl GiftCardProjection {
    pub fn new() -> GiftCardProjection {
        GiftCardProjection {}
    }
    pub fn handle_event(&self, event_message: EventMessage) -> HandlerResult {
        println!("received: {:?}", event_message);
        HandlerResult::EventSuccess
    }
}

pub fn event_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("events"))
        .and(warp::body::json())
        .map(|event: EventMessage| PROJECTION.handle_event(event).into_json())
}

pub async fn register_gift_card_event_handler(configuration: &Configuration, context: &str) {
    let registration = EventHandlerRegistration {
        names: vec![
            String::from(GiftCardIssued::name()),
            String::from(GiftCardRedeemed::name()),
            String::from(GiftCardCanceled::name()),
        ],
        endpoint: String::from("http://host.docker.internal:3030/commands"),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(String::from(CLIENT_ID)),
        component_name: None,
        batch_size: None,
        start: None,
        enabled: None,
        context: None,
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    let result = register_event_handler(configuration, context, Some(registration))
        .await
        .unwrap();
    println!("Result of registering event handlers: {:?}", result)
}
