use OrderCommand::{CancelOrder, CreateOrder, UpdateOrder};
use OrderCommand2::{CancelOrder2, UpdateOrder2};
use OrderEvent::{OrderCancelled, OrderCreated, OrderUpdated};

use crate::decider::{Decider, EventComputation, StateComputation};
use crate::decider_example::OrderCommand2::CreateOrder2;

#[derive(Debug)]
pub enum OrderCommand {
    CreateOrder(CreateOrderCommand),
    UpdateOrder(UpdateOrderCommand),
    CancelOrder(CancelOrderCommand),
}

#[derive(Debug)]
pub enum OrderCommand2 {
    CreateOrder2(CreateOrderCommand),
    UpdateOrder2(UpdateOrderCommand),
    CancelOrder2(CancelOrderCommand),
}

#[derive(Debug)]
pub struct CreateOrderCommand {
    pub order_id: u32,
    pub customer_name: String,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub struct UpdateOrderCommand {
    pub order_id: u32,
    pub new_items: Vec<String>,
}

#[derive(Debug)]
pub struct CancelOrderCommand {
    pub order_id: u32,
}

#[derive(Debug, Clone)]
pub enum OrderEvent {
    OrderCreated(OrderCreatedEvent),
    OrderUpdated(OrderUpdatedEvent),
    OrderCancelled(OrderCancelledEvent),
}

#[derive(Debug, Clone)]
pub struct OrderCreatedEvent {
    pub order_id: u32,
    pub customer_name: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OrderUpdatedEvent {
    pub order_id: u32,
    pub updated_items: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct OrderCancelledEvent {
    pub order_id: u32,
}

#[derive(Debug)]
pub struct OrderState {
    pub order_id: u32,
    pub customer_name: String,
    pub items: Vec<String>,
    pub is_cancelled: bool,
}

#[derive(Debug)]
pub struct OrderState2 {
    pub order_id: u32,
    pub customer_name: String,
    pub items: Vec<String>,
    pub is_cancelled: bool,
}

impl OrderState {
    pub fn clone_state(&self) -> Self {
        OrderState {
            order_id: self.order_id,
            customer_name: self.customer_name.to_owned(),
            items: self.items.to_owned(),
            is_cancelled: self.is_cancelled,
        }
    }
}

impl OrderState2 {
    pub fn clone_state(&self) -> Self {
        OrderState2 {
            order_id: self.order_id,
            customer_name: self.customer_name.to_owned(),
            items: self.items.to_owned(),
            is_cancelled: self.is_cancelled,
        }
    }
}

fn map_to_order_command(order_command2: &OrderCommand2) -> OrderCommand {
    match order_command2 {
        CreateOrder2(create_order) => OrderCommand::CreateOrder(CreateOrderCommand {
            order_id: create_order.order_id,
            customer_name: create_order.customer_name.to_owned(),
            items: create_order.items.to_owned(),
        }),
        UpdateOrder2(update_order) => OrderCommand::UpdateOrder(UpdateOrderCommand {
            order_id: update_order.order_id,
            new_items: update_order.new_items.to_owned(),
        }),
        CancelOrder2(cancel_order) => OrderCommand::CancelOrder(CancelOrderCommand {
            order_id: cancel_order.order_id,
        }),
    }
}

fn map_state_2(order_state: &OrderState) -> OrderState2 {
    OrderState2 {
        order_id: order_state.order_id,
        customer_name: order_state.customer_name.to_owned(),
        items: order_state.items.to_owned(),
        is_cancelled: order_state.is_cancelled,
    }
}

fn map_state_3(order_state: &OrderState) -> OrderState {
    order_state.clone_state()
}

fn map_state_1(order_state: &OrderState2) -> OrderState {
    OrderState {
        order_id: order_state.order_id,
        customer_name: order_state.customer_name.to_owned(),
        items: order_state.items.to_owned(),
        is_cancelled: order_state.is_cancelled,
    }
}

pub fn decider<'a>() -> Decider<'a, OrderCommand, OrderState, OrderEvent> {
    Decider {
        decide: Box::pin(|command, state| {
            match command {
                CreateOrder(create_cmd) => {
                    vec![OrderCreated(OrderCreatedEvent {
                        order_id: create_cmd.order_id,
                        customer_name: create_cmd.customer_name.to_owned(),
                        items: create_cmd.items.to_owned(),
                    })]
                }
                UpdateOrder(update_cmd) => {
                    if state.order_id == update_cmd.order_id {
                        vec![OrderUpdated(OrderUpdatedEvent {
                            order_id: update_cmd.order_id,
                            updated_items: update_cmd.new_items.to_owned(),
                        })]
                    } else {
                        vec![]
                    }
                }
                CancelOrder(cancel_cmd) => {
                    if state.order_id == cancel_cmd.order_id {
                        vec![OrderCancelled(OrderCancelledEvent {
                            order_id: cancel_cmd.order_id,
                        })]
                    } else {
                        vec![]
                    }
                }
            }
        }),
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
        initial_state: Box::pin(|| OrderState {
            order_id: 0,
            customer_name: "".to_string(),
            items: Vec::new(),
            is_cancelled: false,
        }),
    }
}

pub fn main() {
    println!("Decider example");
    println!("-----------------");
    let decider: Decider<OrderCommand, OrderState, OrderEvent> = decider();
    let create_order_command = CreateOrder(CreateOrderCommand {
        order_id: 1,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    let new_events = decider.compute_new_events(&vec![], &create_order_command);
    println!("New Events: {:?}", new_events);
    let new_state = decider.compute_new_state(None, &create_order_command);
    println!("New State: {:?}", new_state);

    let cancel_command = CancelOrder(CancelOrderCommand {
        order_id: 1
    });
    let new_events = decider.compute_new_events(&new_events, &cancel_command);
    println!("New Events: {:?}", new_events);
    let new_state = decider.compute_new_state(Some(new_state), &cancel_command);
    println!("New State: {:?}", new_state);

    // #### Map the decider to OrderCommand2 command ####
    let decider = decider.map_command(&map_to_order_command);
    let create_order_command2 = CreateOrder2(CreateOrderCommand {
        order_id: 2,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    let new_events = decider.compute_new_events(&vec![], &create_order_command2);
    println!("New Events: {:?}", new_events);

    // #### Map the decider to OrderState2 state ####
    let decider = decider.map_state(&map_state_1, &map_state_2);
    let create_order_command2 = CreateOrder2(CreateOrderCommand {
        order_id: 2,
        customer_name: "John Doe".to_string(),
        items: vec!["Item 1".to_string(), "Item 2".to_string()],
    });
    let new_events = decider.compute_new_events(&vec![], &create_order_command2);
    println!("New Events: {:?}", new_events);
}
