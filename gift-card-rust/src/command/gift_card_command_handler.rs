use fmodel_rust::decider::Decider;

use crate::gift_card_api::{
    GiftCardCanceled, GiftCardCommand, GiftCardEvent, GiftCardIssued, GiftCardRedeemed,
};

/// Represents the state of the `GiftCard` decider/aggregate.
#[derive(Debug, Clone, PartialEq)]
pub struct GiftCardState {
    id: String,
    amount: u32,
    is_cancelled: bool,
}

/// Decider is a datatype/struct that represents the main decision-making algorithm. It belongs to the Domain layer.
pub fn decider<'a>() -> Decider<'a, GiftCardCommand, GiftCardState, GiftCardEvent> {
    Decider {
        // Decide new events based on the current state and the command
        decide: Box::new(|command, state| match command {
            // Exhaustive pattern matching on the command
            GiftCardCommand::Issue(cmd) => {
                if state.id == *"0" {
                    vec![GiftCardEvent::Issue(GiftCardIssued {
                        id: cmd.id.to_owned(),
                        amount: cmd.amount,
                    })]
                } else {
                    vec![]
                }
            }
            GiftCardCommand::Redeem(cmd) => {
                if state.id == cmd.id {
                    vec![GiftCardEvent::Redeem(GiftCardRedeemed {
                        id: cmd.id.to_owned(),
                        amount: cmd.amount,
                    })]
                } else {
                    vec![]
                }
            }
            GiftCardCommand::Cancel(cmd) => {
                if state.id == cmd.id {
                    vec![GiftCardEvent::Cancel(GiftCardCanceled {
                        id: cmd.id.to_owned(),
                    })]
                } else {
                    vec![]
                }
            }
        }),
        // Evolve the state based on the current state and the event
        evolve: Box::new(|state, event| {
            let mut new_state = state.clone();
            // Exhaustive pattern matching on the event
            match event {
                GiftCardEvent::Issue(evt) => {
                    new_state.id = evt.id.to_owned();
                    new_state.amount = evt.amount;
                    new_state.is_cancelled = false;
                }
                GiftCardEvent::Redeem(evt) => {
                    new_state.amount -= evt.amount;
                }
                GiftCardEvent::Cancel(_) => {
                    new_state.is_cancelled = true;
                }
            }
            new_state
        }),
        // Initial state of the decider
        initial_state: Box::new(|| GiftCardState {
            id: "0".to_string(),
            amount: 0,
            is_cancelled: false,
        }),
    }
}
