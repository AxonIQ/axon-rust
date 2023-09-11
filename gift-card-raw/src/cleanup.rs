use crate::{CLIENT_ID, CONFIGURATION, CONTEXT};
use synapse_client::apis::command_handlers_api::{
    list_command_handlers, unregister_command_handler,
};
use synapse_client::apis::event_handlers_api::{list_event_handlers, unregister_event_handler};

pub async fn remove_current_event_handlers() {
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

pub async fn remove_current_command_handlers() {
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
        "Unregistered all command handlers with client id: {}",
        CLIENT_ID
    )
}
