use crate::command_handling::AggregateState::{Empty, Present};
use crate::messages::commands::{
    to_command_message, CancelGiftCard, ContainsGiftCardCommand, IssueGiftCard, RedeemGiftCard,
};
use crate::messages::AxonMessage;
use crate::warp_util::HandlerResult;
use crate::{CLIENT_ID, CONFIGURATION, CONTEXT};
use once_cell::sync::Lazy;
use synapse_client::apis::aggregate_api::read_aggregate_events;
use synapse_client::apis::command_handlers_api::register_command_handler;
use synapse_client::apis::commands_api::send_command_message;
use synapse_client::models::{CommandHandlerRegistration, CommandMessage, ListOfEventMessages};
use warp::reply::{Json, WithStatus};
use warp::Filter;

static COMMAND_MODEL: Lazy<GiftCardCommandModel> = Lazy::new(GiftCardCommandModel::new);

pub struct GiftCardCommandModel {}

impl GiftCardCommandModel {
    pub fn new() -> GiftCardCommandModel {
        GiftCardCommandModel {}
    }
    pub async fn handle_command(&self, command_message: CommandMessage) -> HandlerResult {
        println!("received command: {:?}", command_message);
        let gift_card_command = command_message.get_gift_card_command();
        println!("turned to gift card command: {:?}", gift_card_command);
        let aggregate_id = gift_card_command.unwrap().get_aggregate_id();
        println!("used aggregate id: {}", aggregate_id);
        let list = aggregate_events(aggregate_id.as_str()).await;
        let state = into_aggregate_state(list);
        println!("state: {:?}", state);
        HandlerResult::CommandSuccess(command_message)
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
        component_name: None,
        load_factor: None,
        concurrency: None,
        enabled: None,
        context: None,
        client_authentication_id: None,
        server_authentication_id: None,
        last_error: None,
    };
    let result = register_command_handler(&CONFIGURATION, CONTEXT, Some(registration))
        .await
        .unwrap();
    println!("Result of registering command handlers: {:?}", result)
}

pub async fn issue_card() {
    let command = IssueGiftCard {
        id: String::from("0002"),
        amount: 1000,
    };
    let command_message =
        to_command_message(IssueGiftCard::name(), Some(String::from("0002")), &command);
    let result = send_command_message(&CONFIGURATION, CONTEXT, Some(command_message)).await;
    println!("Result of sending a command: {:?}", result)
}

pub async fn aggregate_events(aggregate_id: &str) -> ListOfEventMessages {
    read_aggregate_events(&CONFIGURATION, CONTEXT, aggregate_id)
        .await
        .unwrap()
}

#[derive(Debug)]
enum AggregateState {
    Empty,
    Present(CardSummaryAggregate),
}

#[derive(Debug)]
struct CardSummaryAggregate {
    remaining_amount: u32,
    canceled: bool,
    sequence_identifier: u64,
}

fn into_aggregate_state(list: ListOfEventMessages) -> AggregateState {
    println!("current list: {:?}", list);
    match list.items {
        None => Empty,
        Some(l) if l.is_empty() => Empty,
        Some(l) => {
            let first = l.first().unwrap();
            println!("first: {:?}", first);
            let mut aggregate = CardSummaryAggregate {
                remaining_amount: 0,
                canceled: false,
                sequence_identifier: 0,
            };
            Present(aggregate)
        }
    }
}
