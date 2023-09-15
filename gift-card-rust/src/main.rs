use rocket::{launch, routes};
use synapse_client::apis::configuration;

use fmodel_rust::aggregate::EventSourcedAggregate;

use crate::gift_card_aggregate_controller::{commands, register_gift_card_command_handler};
use crate::gift_card_controller::gift_card_commands;
use crate::gift_card_event_repository::AxonServerEventRepository;

mod gift_card_api;
mod gift_card_command_handler;
mod gift_card_event_repository;
mod gift_card_aggregate_controller;
mod gift_card_command_gateway;
mod gift_card_controller;

#[launch]
async fn rocket() -> _ {
    // Configure the client
    let configuration = configuration::Configuration::default();
    // Set the Axon Server context/db schema
    let context = "default".to_string();
    // Create the aggregate/event repository
    let repository = AxonServerEventRepository {
        configuration: configuration.clone(),
        context: context.clone(),
    };
    // Create the aggregate
    let aggregate = EventSourcedAggregate::new(
        repository,
        gift_card_command_handler::decider(),
    );
    // Create the rocket instance and mount the routes
    let rocket = rocket::build()
        .manage(aggregate)
        .manage(configuration.clone())
        .manage(context.clone())
        .mount("/", routes![commands, gift_card_commands]);

    // Call your service(s) or perform post-launch tasks here: register command handlers, register event handlers, etc.
    register_gift_card_command_handler(&configuration, &context, &"gift_card_client".to_string(), &"gift_card_component".to_string()).await;

    // Return the rocket instance
    rocket
}