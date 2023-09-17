use fmodel_rust::view::View;

use crate::gift_card_api::GiftCardEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct GiftCardViewState {
    id: String,
    amount: u32,
    is_cancelled: bool,
}

#[allow(dead_code)]
pub fn view<'a>() -> View<'a, GiftCardViewState, GiftCardEvent> {
    View {
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
        initial_state: Box::new(|| GiftCardViewState {
            id: "0".to_string(),
            amount: 0,
            is_cancelled: false,
        }),
    }
}
