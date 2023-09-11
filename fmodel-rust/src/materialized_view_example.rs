use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::sync::Mutex;

use async_trait::async_trait;

use OrderEvent::{OrderCancelled, OrderCreated, OrderUpdated};

use crate::decider_example::{OrderCancelledEvent, OrderCreatedEvent, OrderEvent, OrderUpdatedEvent};
use crate::materialized_view::{MaterializedView, ViewStateRepository};
use crate::materialized_view_example::MaterializedViewError::{FetchStateError, SaveStateError};
use crate::view_example::OrderViewState;

/// Error type for the application/materialized view
#[derive(Debug)]
pub enum MaterializedViewError {
    FetchStateError(String),
    SaveStateError(String),
}

impl Error for MaterializedViewError {}

impl Display for MaterializedViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchStateError(message) => {
                write!(f, "Error fetching state: {}", message)
            }
            SaveStateError(message) => {
                write!(f, "Error saving state: {}", message)
            }
        }
    }
}

pub struct InMemoryViewOrderStateRepository {
    states: Mutex<HashMap<u32, OrderViewState>>,
}

impl InMemoryViewOrderStateRepository {
    pub fn new() -> Self {
        InMemoryViewOrderStateRepository {
            states: Mutex::new(HashMap::new()),
        }
    }
}

// Implementation of [ViewStateRepository] for [InMemoryViewOrderStateRepository]
#[async_trait]
impl ViewStateRepository<OrderEvent, OrderViewState> for InMemoryViewOrderStateRepository {
    type Error = MaterializedViewError;

    async fn fetch_state(&self, event: &OrderEvent) -> Result<Option<OrderViewState>, MaterializedViewError> {
        match event {
            OrderCreated(order_created) => {
                let order_id = order_created.order_id;
                Ok(
                    self.states.lock().unwrap()
                        .get(&order_id)
                        .map(|state| state.clone())
                )
            }
            OrderUpdated(order_updated) => {
                let order_id = order_updated.order_id;
                Ok(
                    self.states.lock().unwrap()
                        .get(&order_id)
                        .map(|state| state.clone())
                )
            }
            OrderCancelled(order_canceled) => {
                let order_id = order_canceled.order_id;
                Ok(
                    self.states.lock().unwrap()
                        .get(&order_id)
                        .map(|state| state.clone())
                )
            }
        }
    }
    async fn save(&self, state: &OrderViewState) -> Result<OrderViewState, MaterializedViewError> {
        self.states.lock().unwrap().insert(state.order_id, state.clone());
        Ok(state.clone())
    }
}

pub async fn main() {
    println!("Materialized View example");
    println!("-----------------");
    let repository = InMemoryViewOrderStateRepository::new();
    let materialized_view: MaterializedView<OrderViewState, OrderEvent, InMemoryViewOrderStateRepository, MaterializedViewError> = MaterializedView {
        repository,
        view: crate::view_example::view(),
    };

    let event = OrderCreated(OrderCreatedEvent {
        order_id: 1,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    println!("--");
    println!("Handling event: {:?}", event);
    let result = materialized_view.handle(&event).await;
    println!("Result: {:?}", result);
    println!("States in the repository: {:?}", materialized_view.repository.states);


    let event = OrderUpdated(OrderUpdatedEvent {
        order_id: 1,
        updated_items: vec!["Item 3".to_string(), "Item 4".to_string()],
    });
    println!("--");
    println!("Handling event: {:?}", event);
    let result = materialized_view.handle(&event).await;
    println!("Result: {:?}", result);
    println!("States in the repository: {:?}", materialized_view.repository.states);

    let event = OrderCancelled(OrderCancelledEvent {
        order_id: 1,
    });
    println!("--");
    println!("Handling event: {:?}", event);
    let result = materialized_view.handle(&event).await;
    println!("Result: {:?}", result);
    println!("States in the repository: {:?}", materialized_view.repository.states);
}