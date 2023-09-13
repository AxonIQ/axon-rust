use crate::messages::events::{
    ContainsGiftCardEvent, GiftCardCanceled, GiftCardEvent, GiftCardIssued, GiftCardRedeemed,
};
use crate::messages::queries::{
    ContainsGiftCardQuery, FetchGiftCardSummaries, FetchGiftCardSummary, GiftCardQuery,
    GiftCardSummary,
};
use crate::messages::AxonMessage;
use crate::warp_util::HandlerResult;
use crate::{CLIENT_ID, CONFIGURATION, CONTEXT};
use dashmap::DashMap;
use elsa::sync::FrozenVec;
use once_cell::sync::Lazy;
use synapse_client::apis::event_handlers_api::register_event_handler;
use synapse_client::apis::query_handlers_api::register_query_handler;
use synapse_client::models::{
    EventHandlerRegistration, ListOfEventMessages, QueryHandlerRegistration, QueryMessage,
};
use warp::reply::{Json, WithStatus};
use warp::Filter;

static PROJECTION: Lazy<GiftCardProjection> = Lazy::new(GiftCardProjection::new);

pub struct GiftCardProjection {
    gift_cards: DashMap<String, GiftCardSummary>,
    keys: FrozenVec<String>,
}

impl GiftCardProjection {
    fn new() -> GiftCardProjection {
        GiftCardProjection {
            gift_cards: DashMap::new(),
            keys: FrozenVec::new(),
        }
    }
    async fn handle_events(&self, events: ListOfEventMessages) -> HandlerResult {
        log::debug!("received events: {:?}", events);
        for e in events.items.unwrap() {
            let date_time = e.date_time.clone().unwrap();
            match e.get_gift_card_event().unwrap() {
                GiftCardEvent::Issue(i) => self.handle_issued(i, date_time),
                GiftCardEvent::Redeem(r) => self.handle_redeemed(r, date_time),
                GiftCardEvent::Cancel(c) => self.handle_canceled(c, date_time),
            }
        }
        HandlerResult::EventSuccess
    }
    async fn handle_query(&self, query: QueryMessage) -> HandlerResult {
        log::debug!("received query: {:?}", query);
        let v = match query.get_gift_card_query().unwrap() {
            GiftCardQuery::One(o) => self.query_one(o),
            GiftCardQuery::Multiple(m) => self.query_multiple(m),
        };
        HandlerResult::QuerySuccess(v)
    }
    fn handle_issued(&self, event: GiftCardIssued, date_time: String) {
        self.keys.push(event.id.clone());
        self.gift_cards
            .insert(event.id.clone(), GiftCardSummary::new(event, date_time));
    }
    fn handle_redeemed(&self, event: GiftCardRedeemed, date_time: String) {
        self.gift_cards
            .alter(&event.id.clone(), |_, v| v.redeem(event, date_time))
    }
    fn handle_canceled(&self, event: GiftCardCanceled, date_time: String) {
        self.gift_cards
            .alter(&event.id.clone(), |_, v| v.cancel(event, date_time))
    }
    fn query_one(&self, query: FetchGiftCardSummary) -> serde_json::Value {
        let r = match self.gift_cards.get(&*query.id) {
            None => serde_json::Value::Object(serde_json::Map::new()),
            Some(s) => serde_json::to_value(s.value()).unwrap(),
        };
        log::info!("sending back: {:?}", r);
        r
    }
    fn query_multiple(&self, query: FetchGiftCardSummaries) -> serde_json::Value {
        let mut result_list = vec![];
        let max_size = query.limit as usize;
        let mut index = query.offset as usize;
        log::info!("item in list: {:?}", self.keys.get(0));
        while result_list.len() != max_size {
            match self.keys.get(index) {
                Some(key) => {
                    log::info!("found");
                    let item = self.gift_cards.get(key).unwrap();
                    result_list.push(serde_json::to_value(item.value()).unwrap());
                    index += 1;
                }
                None => {
                    log::info!("not found");
                    break;
                }
            }
        }
        let items = serde_json::Value::Array(result_list);
        log::info!("items in list: {}", items);
        let mut map = serde_json::Map::new();
        map.insert(String::from("items"), items);
        map.insert(
            String::from("total"),
            serde_json::Value::Number(serde_json::Number::from(self.keys.len())),
        );
        map.insert(
            String::from("offset"),
            serde_json::Value::Number(serde_json::Number::from(query.offset)),
        );
        map.insert(
            String::from("limit"),
            serde_json::Value::Number(serde_json::Number::from(query.limit)),
        );
        serde_json::Value::Object(map)
    }
}

impl GiftCardSummary {
    fn new(event: GiftCardIssued, date_time: String) -> GiftCardSummary {
        GiftCardSummary {
            id: event.id,
            initial_amount: event.amount,
            remaining_amount: event.amount,
            canceled: false,
            issued: date_time.clone(),
            last_updated: date_time,
        }
    }
    fn redeem(&self, event: GiftCardRedeemed, date_time: String) -> GiftCardSummary {
        GiftCardSummary {
            id: event.id,
            initial_amount: self.initial_amount,
            remaining_amount: self.remaining_amount - event.amount,
            canceled: self.canceled,
            issued: self.issued.clone(),
            last_updated: date_time,
        }
    }
    fn cancel(&self, event: GiftCardCanceled, date_time: String) -> GiftCardSummary {
        GiftCardSummary {
            id: event.id,
            initial_amount: self.initial_amount,
            remaining_amount: self.remaining_amount,
            canceled: true,
            issued: self.issued.clone(),
            last_updated: date_time,
        }
    }
}

pub fn event_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("events"))
        .and(warp::body::bytes())
        .and_then(|buf: warp::hyper::body::Bytes| async move {
            let events = serde_json::from_slice::<ListOfEventMessages>(&buf).unwrap();
            let result = PROJECTION.handle_events(events).await;
            Ok::<WithStatus<Json>, warp::Rejection>(result.into_json())
        })
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
    log::info!("Result of registering event handler: {:?}", result)
}

pub fn query_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("queries"))
        .and(warp::body::json())
        .and_then(|query: QueryMessage| async move {
            let result = PROJECTION.handle_query(query).await;
            Ok::<WithStatus<Json>, warp::Rejection>(result.into_json())
        })
}

pub async fn register_gift_card_query_handler() {
    let registration = QueryHandlerRegistration {
        names: vec![
            String::from(FetchGiftCardSummary::name()),
            String::from(FetchGiftCardSummaries::name()),
        ],
        endpoint: String::from("http://host.docker.internal:3030/queries"),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(String::from(CLIENT_ID)),
        component_name: Some(String::from("Gift Card Projection")),
        enabled: Some(true),
        context: Some(String::from(CONTEXT)),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    let result = register_query_handler(&CONFIGURATION, CONTEXT, Some(registration))
        .await
        .unwrap();
    log::info!("Result of registering query handler: {:?}", result)
}
