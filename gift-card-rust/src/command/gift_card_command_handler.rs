use fmodel_rust::decider::Decider;

use crate::gift_card_api::{GiftCardCanceled, GiftCardCommand, GiftCardEvent, GiftCardIssued, GiftCardRedeemed};

#[derive(Debug, Clone, PartialEq)]
pub struct GiftCardState {
    id: String,
    amount: u32,
    is_cancelled: bool,
}

pub fn decider<'a>() -> Decider<'a, GiftCardCommand, GiftCardState, GiftCardEvent> {
    Decider {
        decide: Box::new(|command, state| {
            match command {
                GiftCardCommand::Issue(cmd) => {
                    if state.id == *"0" {
                        vec![GiftCardEvent::Issue(GiftCardIssued {
                            id: cmd.id.to_owned(),
                            amount: cmd.amount,
                        })]
                    } else { vec![] }
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
            }
        }),
        evolve: Box::new(|state, event| {
            let mut new_state = state.clone();
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
        initial_state: Box::new(|| GiftCardState {
            id: "0".to_string(),
            amount: 0,
            is_cancelled: false,
        }),
    }
}