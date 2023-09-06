use crate::command_handling::{command_route, issue_card, register_gift_card_command_handler};
use crate::projection::{event_route, register_gift_card_event_handler};
use once_cell::sync::Lazy;
use synapse_client::apis::command_handlers_api::{
    list_command_handlers, unregister_command_handler,
};
use synapse_client::apis::configuration::Configuration;
use synapse_client::apis::event_handlers_api::{list_event_handlers, unregister_event_handler};
use tokio::join;
use tokio::time::{sleep, Duration};
use warp::Filter;

mod command_handling;
mod messages;
mod projection;
mod warp_util;

static CONFIGURATION: Lazy<Configuration> = Lazy::new(Configuration::new);
static CONTEXT: &str = "default";
static CLIENT_ID: &str = "gift-card-raw";

#[tokio::main]
async fn main() {
    remove_current_event_handlers().await;
    remove_current_command_handlers().await;
    let handle = tokio::spawn(async move {
        start_listening().await;
    });
    register_gift_card_command_handler(&CONFIGURATION, CONTEXT).await;
    register_gift_card_event_handler(&CONFIGURATION, CONTEXT).await;
    sleep(Duration::from_secs(3)).await;
    issue_card(&CONFIGURATION, CONTEXT).await;
    join!(handle).0.unwrap();
}

pub async fn start_listening() {
    let routes = command_route().or(event_route());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn remove_current_event_handlers() {
    let event_handler_list = list_event_handlers(&CONFIGURATION, CONTEXT).await.unwrap();
    for event_handler in event_handler_list.items.unwrap() {
        if let Some(client_id) = event_handler.client_id {
            if client_id == CLIENT_ID {
                println!(
                    "Unregistering previous handler with id: {}",
                    event_handler.id
                );
                unregister_event_handler(&CONFIGURATION, CONTEXT, &event_handler.id)
                    .await
                    .unwrap();
            }
        }
    }
    println!(
        "Unregistered all event handlers with client id: {}",
        CLIENT_ID
    )
}

async fn remove_current_command_handlers() {
    let command_handler_list = list_command_handlers(&CONFIGURATION, CONTEXT)
        .await
        .unwrap();
    for command_handler in command_handler_list.items.unwrap() {
        if let Some(client_id) = command_handler.client_id {
            if client_id == CLIENT_ID {
                println!(
                    "Unregistering previous handler with id: {}",
                    command_handler.id
                );
                unregister_command_handler(&CONFIGURATION, CONTEXT, &command_handler.id)
                    .await
                    .unwrap();
            }
        }
    }
    println!(
        "Unregistered all command handlers wiht client id: {}",
        CLIENT_ID
    )
}
