use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::sync::Mutex;

use async_trait::async_trait;

use OrderCommand::{CancelOrder, CreateOrder, UpdateOrder};
use OrderEvent::{OrderCancelled, OrderCreated, OrderUpdated};

use crate::aggregate::{EventRepository, EventSourcedAggregate};
use crate::aggregate_example::AggregateError::{FetchEventsError, FetchStateError, SaveEventsError, SaveStateError};
use crate::decider_example::{CancelOrderCommand, CreateOrderCommand, OrderCommand, OrderEvent, OrderState, UpdateOrderCommand};

/// Error type for the application/aggregate
#[derive(Debug)]
pub enum AggregateError {
    FetchEventsError(String),
    SaveEventsError(String),
    FetchStateError(String),
    SaveStateError(String),
}


impl Error for AggregateError {}

impl Display for AggregateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchEventsError(message) => {
                write!(f, "Error fetching events: {}", message)
            }
            SaveEventsError(message) => {
                write!(f, "Error saving events: {}", message)
            }
            FetchStateError(message) => {
                write!(f, "Error fetching state: {}", message)
            }
            SaveStateError(message) => {
                write!(f, "Error saving state: {}", message)
            }
        }
    }
}


pub struct InMemoryOrderEventRepository {
    events: Mutex<Vec<OrderEvent>>,
}

impl InMemoryOrderEventRepository {
    pub fn new() -> Self {
        InMemoryOrderEventRepository {
            events: Mutex::new(vec![]),
        }
    }
}

// Implementation of [EventRepository] for [InMemoryOrderEventRepository]
#[async_trait]
impl EventRepository<OrderCommand, OrderEvent> for InMemoryOrderEventRepository {
    type Error = AggregateError;

    async fn fetch_events(&self, command: &OrderCommand) -> Result<Vec<OrderEvent>, AggregateError> {
        match command {
            CreateOrder(create_order) => {
                let order_id = create_order.order_id;
                Ok(
                    self.events
                        .lock()
                        .unwrap()
                        .clone()
                        .into_iter()
                        .filter(|event| {
                            if let OrderUpdated(order_updated) = event {
                                order_updated.order_id.eq(&order_id)
                            } else if let OrderCreated(order_created) = event {
                                order_created.order_id.eq(&order_id)
                            } else if let OrderCancelled(order_cancelled) = event {
                                order_cancelled.order_id.eq(&order_id)
                            } else {
                                false
                            }
                        })
                        .collect()
                )
            }
            UpdateOrder(update_order) => {
                let order_id = update_order.order_id;
                Ok(
                    self.events
                        .lock()
                        .unwrap()
                        .clone()
                        .into_iter()
                        .filter(|event| {
                            if let OrderUpdated(order_updated) = event {
                                order_updated.order_id.eq(&order_id)
                            } else if let OrderCreated(order_created) = event {
                                order_created.order_id.eq(&order_id)
                            } else if let OrderCancelled(order_cancelled) = event {
                                order_cancelled.order_id.eq(&order_id)
                            } else {
                                false
                            }
                        })
                        .collect())
            }
            CancelOrder(cancel_order) => {
                let order_id = cancel_order.order_id;
                Ok(
                    self.events
                        .lock()
                        .unwrap()
                        .clone()
                        .into_iter()
                        .filter(|event| {
                            if let OrderUpdated(order_updated) = event {
                                order_updated.order_id.eq(&order_id)
                            } else if let OrderCreated(order_created) = event {
                                order_created.order_id.eq(&order_id)
                            } else if let OrderCancelled(order_cancelled) = event {
                                order_cancelled.order_id.eq(&order_id)
                            } else {
                                false
                            }
                        })
                        .collect())
            }
        }
    }

    async fn save(&self, events: &Vec<OrderEvent>) -> Result<Vec<OrderEvent>, AggregateError> {
        self.events
            .lock()
            .unwrap()
            .extend_from_slice(&events);
        Ok(events.clone())
    }
}

pub async fn main() {
    println!("Aggregate example");
    println!("-----------------");
    let repository = InMemoryOrderEventRepository::new();
    let aggregate: EventSourcedAggregate<OrderCommand, OrderState, OrderEvent, InMemoryOrderEventRepository, AggregateError> = EventSourcedAggregate {
        repository,
        decider: crate::decider_example::decider(),
    };

    let command = CreateOrder(CreateOrderCommand {
        order_id: 1,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    println!("--");
    println!("Handling command: {:?}", command);
    let result = aggregate.handle(&command).await;
    println!("Result: {:?}", result);
    println!("Events in the repository: {:?}", aggregate.repository.events);


    let command = UpdateOrder(UpdateOrderCommand {
        order_id: 1,
        new_items: vec!["Item 3".to_string(), "Item 4".to_string()],
    });
    println!("--");
    println!("Handling command: {:?}", command);
    let result = aggregate.handle(&command).await;
    println!("Result: {:?}", result);
    println!("Events in the repository: {:?}", aggregate.repository.events);

    let command = CancelOrder(CancelOrderCommand {
        order_id: 1,
    });
    println!("--");
    println!("Handling command: {:?}", command);
    let result = aggregate.handle(&command).await;
    println!("Result: {:?}", result);
    println!("Events in the repository: {:?}", aggregate.repository.events);
}