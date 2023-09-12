use crate::messages::events::{GiftCardCanceled, GiftCardIssued, GiftCardRedeemed};
use crate::messages::queries::GiftCardSummary;
use crate::messages::AxonMessage;
use crate::warp_util::HandlerResult;
use crate::{CLIENT_ID, CONFIGURATION, CONTEXT};
use dashmap::DashMap;
use elsa::sync::FrozenVec;
use once_cell::sync::Lazy;
use synapse_client::apis::event_handlers_api::register_event_handler;
use synapse_client::models::EventHandlerRegistration;
use warp::hyper::body::Bytes;
use warp::Filter;

static PROJECTION: Lazy<GiftCardProjection> = Lazy::new(GiftCardProjection::new);

pub struct GiftCardProjection {
    gift_cards: DashMap<String, GiftCardSummary>,
    keys: FrozenVec<String>,
}

impl GiftCardProjection {
    pub fn new() -> GiftCardProjection {
        GiftCardProjection {
            gift_cards: DashMap::new(),
            keys: FrozenVec::new(),
        }
    }
    pub fn handle_event(&self, event_message: String) -> HandlerResult {
        println!("received event: {:?}", event_message);
        HandlerResult::EventSuccess
    }
}

pub fn event_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("events"))
        .and(warp::body::bytes())
        .map(|event: Bytes| {
            println!(
                "Request body: {:?}",
                String::from_utf8(event.to_vec()).unwrap()
            );
            String::from("Not working yet..")
        })
        .map(|event: String| PROJECTION.handle_event(event).into_json())
}

pub async fn register_gift_card_event_handler() {
    let registration = EventHandlerRegistration {
        names: vec![
            String::from(GiftCardIssued::name()),
            String::from(GiftCardRedeemed::name()),
            String::from(GiftCardCanceled::name()),
        ],
        endpoint: String::from("http://host.docker.internal:3030/events"),
        endpoint_type: Some(String::from("http-list-of-messages")),
        endpoint_options: None,
        client_id: Some(String::from(CLIENT_ID)),
        component_name: Some(String::from("Gift Card Projection")),
        batch_size: Some(50),
        start: Some(0),
        enabled: Some(true),
        context: Some(String::from(CONTEXT)),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    let result = register_event_handler(&CONFIGURATION, CONTEXT, Some(registration))
        .await
        .unwrap();
    println!("Result of registering event handlers: {:?}", result)
}
