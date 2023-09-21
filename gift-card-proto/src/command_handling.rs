use crate::gift_card::commands::{CancelGiftCard, IssueGiftCard, RedeemGiftCard};
use crate::gift_card::events::{GiftCardCanceled, GiftCardIssued, GiftCardRedeemed};
use crate::messages::commands::{ContainsGiftCardCommand, GiftCardCommand};
use crate::messages::events::{to_publishable_event_message, ContainsGiftCardEvent, GiftCardEvent};
use crate::messages::AxonMessage;
use crate::warp_util::{HandlerErrorMessage, HandlerResult};
use crate::{CLIENT_ID, CONFIGURATION, CONTEXT};
use moka::future::Cache;
use once_cell::sync::Lazy;
use synapse_client::apis::aggregate_api::read_aggregate_events;
use synapse_client::apis::command_handlers_api::register_command_handler;
use synapse_client::apis::events_api::publish_event_message;
use synapse_client::models::{CommandHandlerRegistration, CommandMessage, ListOfEventMessages};
use warp::reply::{Json, WithStatus};
use warp::Filter;

static COMMAND_MODEL: Lazy<GiftCardCommandModel> = Lazy::new(GiftCardCommandModel::new);

pub struct GiftCardCommandModel {
    cache: Cache<String, GiftCardAggregate>,
}

impl GiftCardCommandModel {
    fn new() -> GiftCardCommandModel {
        GiftCardCommandModel {
            cache: Cache::new(100),
        }
    }
    pub async fn handle_command(&self, command_message: CommandMessage) -> HandlerResult {
        log::info!("received command: {:?}", command_message);
        let gift_card_command = command_message.get_gift_card_command().unwrap();
        log::info!("turned to gift card command: {:?}", gift_card_command);
        let aggregate_id = gift_card_command.get_aggregate_id();
        log::info!("used aggregate id: {}", aggregate_id);
        let state = match self.cache.get(&*aggregate_id) {
            None => {
                let list = aggregate_events(aggregate_id.as_str()).await;
                into_aggregate_state(list)
            }
            Some(s) => Some(s),
        };
        log::info!("current state: {:?}", state);
        match gift_card_command.execute(&state) {
            Ok(event) => match publish_event(event, state).await {
                Ok(new_state) => {
                    log::info!("updated state: {:?}", new_state);
                    self.cache.insert(aggregate_id, new_state).await;
                    HandlerResult::CommandSuccess(command_message)
                }
                Err(e) => e,
            },
            Err(e) => {
                self.cache.remove(&*aggregate_id).await;
                HandlerResult::bad_request(e)
            }
        }
    }
}

pub fn command_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("commands"))
        .and(warp::body::json())
        .and_then(|command: CommandMessage| async move {
            let result = COMMAND_MODEL.handle_command(command).await;
            Ok::<WithStatus<Json>, warp::Rejection>(result.into_json())
        })
}

pub async fn register_gift_card_command_handler() {
    let registration = CommandHandlerRegistration {
        names: vec![
            String::from(IssueGiftCard::name()),
            String::from(RedeemGiftCard::name()),
            String::from(CancelGiftCard::name()),
        ],
        endpoint: String::from("http://host.docker.internal:3030/commands"),
        endpoint_type: Some(String::from("http-message")),
        endpoint_options: None,
        client_id: Some(String::from(CLIENT_ID)),
        component_name: Some(String::from("Gift Card Commands Model")),
        load_factor: Some(100),
        concurrency: Some(8),
        enabled: Some(true),
        context: Some(String::from(CONTEXT)),
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    let result = register_command_handler(&CONFIGURATION, CONTEXT, Some(registration))
        .await
        .unwrap();
    log::info!("Result of registering command handler: {:?}", result)
}

pub async fn aggregate_events(aggregate_id: &str) -> ListOfEventMessages {
    read_aggregate_events(&CONFIGURATION, CONTEXT, aggregate_id)
        .await
        .unwrap()
}

#[derive(Debug, Clone)]
struct GiftCardAggregate {
    id: String,
    remaining_amount: u32,
    canceled: bool,
    sequence_identifier: i64,
}

impl GiftCardAggregate {
    fn new(event: &GiftCardIssued) -> GiftCardAggregate {
        GiftCardAggregate {
            id: event.id.clone(),
            remaining_amount: event.amount,
            canceled: false,
            sequence_identifier: 0,
        }
    }
    fn cancel(&mut self) {
        self.canceled = true;
        self.sequence_identifier += 1;
    }
    fn redeem(&mut self, amount: u32) {
        self.remaining_amount -= amount;
        self.sequence_identifier += 1;
    }
}

fn into_aggregate_state(list: ListOfEventMessages) -> Option<GiftCardAggregate> {
    log::info!("current list: {:?}", list);
    match list.items {
        None => None,
        Some(l) if l.is_empty() => None,
        Some(l) => {
            let mut aggregate = None;
            for event in l {
                match event.get_gift_card_event().unwrap().apply(aggregate) {
                    Ok(a) => {
                        aggregate = Some(a);
                    }
                    Err(e) => panic!("Error building aggregate: {}", e),
                }
            }
            aggregate
        }
    }
}

impl GiftCardEvent {
    fn apply(&self, aggregate: Option<GiftCardAggregate>) -> Result<GiftCardAggregate, String> {
        match self {
            GiftCardEvent::Issue(i) => match aggregate {
                None => Ok(GiftCardAggregate::new(i)),
                Some(_) => Err(String::from(
                    "Issue is expected to be only the first event.",
                )),
            },
            GiftCardEvent::Redeem(r) => match aggregate {
                None => Err(String::from(
                    "There should already be an aggregate to redeem.",
                )),
                Some(mut a) => {
                    a.redeem(r.amount);
                    Ok(a)
                }
            },
            GiftCardEvent::Cancel(_) => match aggregate {
                None => Err(String::from(
                    "There should already be an aggregate to cancel.",
                )),
                Some(mut a) => {
                    a.cancel();
                    Ok(a)
                }
            },
        }
    }
}

impl GiftCardCommand {
    fn execute(
        &self,
        optional_aggregate: &Option<GiftCardAggregate>,
    ) -> Result<GiftCardEvent, String> {
        match self {
            GiftCardCommand::Issue(i) => {
                if let Some(a) = optional_aggregate {
                    return Err(format!("Card already exist, current state is: {:?}.", a));
                };
                if i.amount < 10 {
                    Err(format!(
                        "Amount should be at least 10, but was: {}.",
                        i.amount
                    ))
                } else {
                    Ok(GiftCardEvent::Issue(GiftCardIssued {
                        id: i.id.clone(),
                        amount: i.amount,
                    }))
                }
            }
            GiftCardCommand::Redeem(r) => {
                let aggregate = match optional_aggregate {
                    Some(a) => a,
                    None => return Err(String::from("There is not yet a card to redeem from.")),
                };
                if aggregate.canceled {
                    Err(String::from("Card is already canceled."))
                } else if r.amount > aggregate.remaining_amount {
                    Err(format!(
                        "Amount left on the card: {} is less than amount to redeem: {}.",
                        aggregate.remaining_amount, r.amount
                    ))
                } else {
                    Ok(GiftCardEvent::Redeem(GiftCardRedeemed {
                        id: aggregate.id.clone(),
                        amount: r.amount,
                    }))
                }
            }
            GiftCardCommand::Cancel(_) => {
                let aggregate = match optional_aggregate {
                    Some(a) => a,
                    None => return Err(String::from("There is not yet a card to cancel.")),
                };
                if aggregate.canceled {
                    Err(String::from("Card was already canceled."))
                } else if aggregate.remaining_amount == 0 {
                    Err(String::from(
                        "Card has nothing left, so it can't be cancelled",
                    ))
                } else {
                    Ok(GiftCardEvent::Cancel(GiftCardCanceled {
                        id: aggregate.id.clone(),
                    }))
                }
            }
        }
    }
}

async fn publish_event(
    event: GiftCardEvent,
    aggregate: Option<GiftCardAggregate>,
) -> Result<GiftCardAggregate, HandlerResult> {
    let name = event.get_name();
    let payload = event.get_payload();
    let new_aggregate = event.apply(aggregate).unwrap();
    let event_message = to_publishable_event_message(
        name,
        Some(new_aggregate.id.clone()),
        Some(String::from("GiftCardAggregate")),
        Some(new_aggregate.sequence_identifier),
        payload,
    );
    match publish_event_message(&CONFIGURATION, CONTEXT, Some(event_message)).await {
        Ok(_) => Ok(new_aggregate),
        Err(e) => Err(HandlerResult::Error(HandlerErrorMessage {
            code: 500,
            message: e.to_string(),
        })),
    }
}
