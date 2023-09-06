use crate::messages::commands::{CancelGiftCard, IssueGiftCard, RedeemGiftCard};
use crate::messages::AxonMessage;
use crate::warp_util::HandlerResult;
use crate::CLIENT_ID;
use once_cell::sync::Lazy;
use synapse_client::apis::command_handlers_api::register_command_handler;
use synapse_client::apis::commands_api::send_command;
use synapse_client::apis::configuration::Configuration;
use synapse_client::models::{CommandHandlerRegistration, CommandMessage};
use warp::Filter;

static COMMAND_MODEL: Lazy<GiftCardCommandModel> = Lazy::new(GiftCardCommandModel::new);

pub struct GiftCardCommandModel {}

impl GiftCardCommandModel {
    pub fn new() -> GiftCardCommandModel {
        GiftCardCommandModel {}
    }
    pub fn handle_command(&self, command_message: CommandMessage) -> HandlerResult {
        println!("received: {:?}", command_message);
        HandlerResult::CommandSuccess(command_message)
    }
}

pub fn command_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path!("commands"))
        .and(warp::body::json())
        .map(|command: CommandMessage| COMMAND_MODEL.handle_command(command).into_json())
}

pub async fn register_gift_card_command_handler(configuration: &Configuration, context: &str) {
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
    let result = register_command_handler(configuration, context, Some(registration))
        .await
        .unwrap();
    println!("Result of registering command handlers: {:?}", result)
}

pub async fn issue_card(configuration: &Configuration, context: &str) {
    let command = IssueGiftCard {
        id: String::from("0002"),
        amount: 1000,
    };
    let body = serde_json::to_string(&command).unwrap();
    let result = send_command(
        configuration,
        context,
        IssueGiftCard::name(),
        None,
        None,
        None,
        Some(&*command.id),
        Some(body.parse().unwrap()),
    )
    .await;
    println!("Result of sending a command: {:?}", result)
}
