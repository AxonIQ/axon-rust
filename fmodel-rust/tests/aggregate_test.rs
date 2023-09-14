use std::error::Error;
use std::sync::Mutex;

use async_trait::async_trait;
use derive_more::Display;

use fmodel_rust::aggregate::{EventRepository, EventSourcedAggregate};
use fmodel_rust::decider::Decider;

use crate::api::{CancelOrderCommand, CreateOrderCommand, OrderCancelledEvent, OrderCommand, OrderCreatedEvent, OrderEvent, OrderUpdatedEvent, UpdateOrderCommand};

mod api;

/// Error type for the application/aggregate
#[derive(Debug, Display)]
#[allow(dead_code)]
enum AggregateError {
    FetchEvents(String),
    SaveEvents(String),
    FetchState(String),
    SaveState(String),
}

impl Error for AggregateError {}

/// A simple in-memory event repository - infrastructure
struct InMemoryOrderEventRepository {
    events: Mutex<Vec<(OrderEvent, i32)>>,
}

impl InMemoryOrderEventRepository {
    fn new() -> Self {
        InMemoryOrderEventRepository {
            events: Mutex::new(vec![]),
        }
    }
}

/// Implementation of [EventRepository] for [InMemoryOrderEventRepository] - infrastructure
#[async_trait]
impl EventRepository<OrderCommand, OrderEvent> for InMemoryOrderEventRepository {
    type Error = AggregateError;
    type Version = i32;

    async fn fetch_events(&self, command: &OrderCommand) -> Result<Vec<(OrderEvent, i32)>, AggregateError> {
        Ok(
            self.events
                .lock()
                .unwrap()
                .clone()
                .into_iter()
                .filter(|(event, _)| { event.id() == command.id() })
                .collect()
        )
    }

    async fn save(&self, events: &[OrderEvent], latest_version: &Option<i32>) -> Result<Vec<(OrderEvent, i32)>, AggregateError> {
        let mut latest_version = latest_version.to_owned().unwrap_or(-1);
        let events =
            events.into_iter().map(|event| {
                latest_version += 1;
                (event.clone(), latest_version)
            }).collect::<Vec<(OrderEvent, i32)>>();

        self.events
            .lock()
            .unwrap()
            .extend_from_slice(&*events.clone());
        Ok(Vec::from(events))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct OrderState {
    order_id: u32,
    customer_name: String,
    items: Vec<String>,
    is_cancelled: bool,
}

/// Decider for the Order aggregate - Domain logic
fn decider<'a>() -> Decider<'a, OrderCommand, OrderState, OrderEvent> {
    Decider {
        decide: Box::new(|command, state| {
            match command {
                OrderCommand::Create(create_cmd) => {
                    vec![OrderEvent::Created(OrderCreatedEvent {
                        order_id: create_cmd.order_id,
                        customer_name: create_cmd.customer_name.to_owned(),
                        items: create_cmd.items.to_owned(),
                    })]
                }
                OrderCommand::Update(update_cmd) => {
                    if state.order_id == update_cmd.order_id {
                        vec![OrderEvent::Updated(OrderUpdatedEvent {
                            order_id: update_cmd.order_id,
                            updated_items: update_cmd.new_items.to_owned(),
                        })]
                    } else {
                        vec![]
                    }
                }
                OrderCommand::Cancel(cancel_cmd) => {
                    if state.order_id == cancel_cmd.order_id {
                        vec![OrderEvent::Cancelled(OrderCancelledEvent {
                            order_id: cancel_cmd.order_id,
                        })]
                    } else {
                        vec![]
                    }
                }
            }
        }),
        evolve: Box::new(|state, event| {
            let mut new_state = state.clone();
            match event {
                OrderEvent::Created(created_event) => {
                    new_state.order_id = created_event.order_id;
                    new_state.customer_name = created_event.customer_name.to_owned();
                    new_state.items = created_event.items.to_owned();
                }
                OrderEvent::Updated(updated_event) => {
                    new_state.items = updated_event.updated_items.to_owned();
                }
                OrderEvent::Cancelled(_) => {
                    new_state.is_cancelled = true;
                }
            }
            new_state
        }),
        initial_state: Box::new(|| OrderState {
            order_id: 0,
            customer_name: "".to_string(),
            items: Vec::new(),
            is_cancelled: false,
        }),
    }
}

#[tokio::test]
async fn test() {
    let repository = InMemoryOrderEventRepository::new();
    let aggregate: EventSourcedAggregate<OrderCommand, OrderState, OrderEvent, InMemoryOrderEventRepository, i32, AggregateError> = EventSourcedAggregate {
        repository,
        decider: decider(),
    };
    let command = OrderCommand::Create(CreateOrderCommand {
        order_id: 1,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    let result = aggregate.handle(&command).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [
        (OrderEvent::Created(OrderCreatedEvent {
            order_id: 1,
            customer_name: "John Doe".to_string(),
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        }),
         0)]);
    let command = OrderCommand::Update(UpdateOrderCommand {
        order_id: 1,
        new_items: vec!["Item 3".to_string(), "Item 4".to_string()],
    });
    let result = aggregate.handle(&command).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [
        (OrderEvent::Updated(OrderUpdatedEvent {
            order_id: 1,
            updated_items: vec!["Item 3".to_string(), "Item 4".to_string()],
        }),
         1)]);
    let command = OrderCommand::Cancel(CancelOrderCommand {
        order_id: 1,
    });
    let result = aggregate.handle(&command).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), [
        (OrderEvent::Cancelled(OrderCancelledEvent {
            order_id: 1,
        }),
         2)]);
}