use fmodel_rust::aggregate::EventSourcedAggregate;
use fmodel_rust::materialized_view::MaterializedView;
use rocket::{launch, routes};
use synapse_client::apis::configuration;

use crate::command::gift_card_aggregate_controller::commands;
use crate::command::gift_card_aggregate_controller::register_gift_card_command_handler;
use crate::command::gift_card_command_handler::decider;
use crate::command::gift_card_event_repository::AxonServerEventRepository;
use crate::gift_card_controller::gift_card_commands;
use crate::query::gift_card_event_handler::view;
use crate::query::gift_card_materialized_view_controller::events;
use crate::query::gift_card_materialized_view_controller::register_gift_card_event_handler;
use crate::query::gift_card_view_state_repository::InMemoryViewStateRepository;

mod command;
mod gift_card_api;
mod gift_card_controller;
mod query;

#[launch]
async fn rocket() -> _ {
    // Configure the client
    let configuration = configuration::Configuration::default();
    // Set the Axon Server context/db schema
    let context = "default".to_string();

    // ####################
    // ### Command Side ###
    // ####################

    // Create the aggregate event repository
    let repository = AxonServerEventRepository {
        configuration: configuration.clone(),
        context: context.clone(),
    };
    // Create the aggregate
    let aggregate = EventSourcedAggregate {
        repository,
        decider: decider(),
    };
    // ####################
    // ### Query Side ###
    // ####################

    // Create the materialized view state repository
    let view_repository = InMemoryViewStateRepository::new();
    // Create the materialized view
    let materialized_view = MaterializedView {
        repository: view_repository,
        view: view(),
    };
    // Create the rocket instance and mount the routes
    let rocket = rocket::build()
        .manage(aggregate)
        .manage(materialized_view)
        .manage(configuration.clone())
        .manage(context.clone())
        .mount("/", routes![commands, gift_card_commands, events]);

    // Call your service(s) or perform post-launch tasks here: register command handlers, register event handlers, etc.
    register_gift_card_command_handler(
        &configuration,
        &context,
        &"gift_card_client".to_string(),
        &"gift_card_component".to_string(),
        &"http://host.docker.internal:8000".to_string(),
    )
    .await;

    register_gift_card_event_handler(
        &configuration,
        &context,
        &"gift_card_client".to_string(),
        &"gift_card_component".to_string(),
        &"http://host.docker.internal:8000".to_string(),
    )
    .await;

    // Return the rocket instance
    rocket
}
