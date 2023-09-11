use crate::decider_example::{OrderCancelledEvent, OrderCreatedEvent, OrderEvent, OrderUpdatedEvent};
use crate::decider_example::OrderEvent::{OrderCancelled, OrderCreated, OrderUpdated};
use crate::view::View;

#[derive(Debug, Clone)]
pub struct OrderViewState {
    pub order_id: u32,
    pub customer_name: String,
    pub items: Vec<String>,
    pub is_cancelled: bool,
}

impl OrderViewState {
    pub fn clone_state(&self) -> Self {
        OrderViewState {
            order_id: self.order_id,
            customer_name: self.customer_name.to_owned(),
            items: self.items.to_owned(),
            is_cancelled: self.is_cancelled,
        }
    }
}

pub fn view<'a>() -> View<'a, OrderViewState, OrderEvent> {
    View {
        evolve: Box::pin(|state, event| {
            let mut new_state = state.clone_state();
            match event {
                OrderCreated(created_event) => {
                    new_state.order_id = created_event.order_id;
                    new_state.customer_name = created_event.customer_name.to_owned();
                    new_state.items = created_event.items.to_owned();
                }
                OrderUpdated(updated_event) => {
                    new_state.items = updated_event.updated_items.to_owned();
                }
                OrderCancelled(_) => {
                    new_state.is_cancelled = true;
                }
            }
            new_state
        }),
        initial_state: Box::pin(|| OrderViewState {
            order_id: 0,
            customer_name: "".to_string(),
            items: Vec::new(),
            is_cancelled: false,
        }),
    }
}

pub fn main() {
    println!("View example");
    println!("-----------------");
    let view: View<OrderViewState, OrderEvent> = view();
    let order_created_event = OrderCreated(OrderCreatedEvent {
        order_id: 1,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    let new_state = (view.evolve)(&(view.initial_state)(), &order_created_event);
    println!("New State: {:?}", new_state);

    let order_updated_event = OrderUpdated(OrderUpdatedEvent {
        order_id: 1,
        updated_items: vec!["Item 11".to_string(), "Item 22".to_string(), "Item 33".to_string()],
    });
    let new_state = (view.evolve)(&new_state, &order_updated_event);
    println!("New State: {:?}", new_state);

    let order_canceled_event = OrderCancelled(OrderCancelledEvent {
        order_id: 1
    });
    let new_state = (view.evolve)(&new_state, &order_canceled_event);
    println!("New State: {:?}", new_state);
}
