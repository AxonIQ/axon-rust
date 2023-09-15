use rocket::{launch, routes};
use synapse_client::apis::configuration;

use fmodel_rust::aggregate::EventSourcedAggregate;

use crate::aggregate_repository::AxonServerEventRepository;
use crate::aggregate_web::{commands, register_gift_card_command_handler};

mod api;
mod decider;
mod aggregate_repository;
mod aggregate_web;
mod command_gateway;

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
        decider::decider(),
    );
    // Create the rocket instance and mount the routes
    let rocket = rocket::build()
        .manage(aggregate)
        .mount("/", routes![commands]);

    // Call your service(s) or perform post-launch tasks here: register command handlers, register event handlers, etc.
    register_gift_card_command_handler(&configuration, &context, &"gift_card_client".to_string(), &"gift_card_component".to_string()).await;

    // Return the rocket instance
    rocket
}