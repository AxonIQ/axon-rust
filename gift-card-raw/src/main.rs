use crate::cleanup::{
    remove_current_command_handlers, remove_current_event_handlers, remove_current_query_handlers,
};
use crate::command_handling::{command_route, register_gift_card_command_handler};
use crate::projection::{
    event_route, query_route, register_gift_card_event_handler, register_gift_card_query_handler,
};
use crate::rest_api::start_api;
use once_cell::sync::Lazy;
use synapse_client::apis::configuration::Configuration;
use warp::Filter;

mod cleanup;
mod command_handling;
mod messages;
mod projection;
mod rest_api;
mod warp_util;

static CONFIGURATION: Lazy<Configuration> = Lazy::new(Configuration::new);
static CONTEXT: &str = "default";
static CLIENT_ID: &str = "gift-card-raw";

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    remove_current_command_handlers().await;
    remove_current_event_handlers().await;
    remove_current_query_handlers().await;
    let handlers_handle = tokio::spawn(async move {
        start_listening().await;
    });
    let api_handle = tokio::spawn(async move {
        start_api().await;
    });
    register_gift_card_command_handler().await;
    register_gift_card_event_handler().await;
    register_gift_card_query_handler().await;
    tokio::join!(api_handle).0.unwrap();
    tokio::join!(handlers_handle).0.unwrap();
}

async fn start_listening() {
    let routes = command_route().or(event_route()).or(query_route());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
