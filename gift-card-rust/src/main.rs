use synapse_client::apis::configuration;

use fmodel_rust::aggregate::EventSourcedAggregate;

use crate::aggregate_repository::AxonServerEventRepository;
use crate::api::GiftCardCommand;

mod api;
mod decider;
mod aggregate_repository;

#[tokio::main]
async fn main() {
    // Configure the client
    let configuration = configuration::Configuration::default();
    // Set the context/db schema
    let context = "default".to_string();
    // Create the repository
    let repository = AxonServerEventRepository {
        configuration,
        context,
    };
    // Create the aggregate
    let aggregate = EventSourcedAggregate {
        repository,
        decider: decider::decider(),
    };
    // Issue a gift card
    let command = GiftCardCommand::Issue(api::IssueGiftCard {
        id: "1".to_owned(),
        amount: 1000,
    });
    let result = aggregate.handle(&command).await;
    println!("result: {:?}", result);
    // Redeem a gift card
    let command = GiftCardCommand::Redeem(api::RedeemGiftCard {
        id: "1".to_owned(),
        amount: 50,
    });
    let result = aggregate.handle(&command).await;
    println!("result: {:?}", result);
    // Cancel a gift card
    let command = GiftCardCommand::Cancel(api::CancelGiftCard {
        id: "1".to_owned(),
    });
    let result = aggregate.handle(&command).await;
    println!("result: {:?}", result);
}
