use fmodel_rust::view::View;
use serde_derive::{Deserialize, Serialize};

use crate::gift_card_api::GiftCardEvent;

/// View state for the GiftCard view/materialized view
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GiftCardViewState {
    pub id: String,
    pub amount: u32,
    pub is_cancelled: bool,
}

/// View is a datatype that represents the event handling algorithm, responsible for translating the events into denormalized state, which is more adequate for querying. It belongs to the Domain layer.
#[allow(dead_code)]
pub fn view<'a>() -> View<'a, GiftCardViewState, GiftCardEvent> {
    View {
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
        // Initial state of the view
        initial_state: Box::new(|| GiftCardViewState {
            id: "0".to_string(),
            amount: 0,
            is_cancelled: false,
        }),
    }
}
