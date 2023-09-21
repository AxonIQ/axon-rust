use crate::gift_card::events::{GiftCardCanceled, GiftCardIssued, GiftCardRedeemed};
use crate::messages::{message_to_payload, value_to_message, AxonMessage};
use serde_json::Value;
use synapse_client::models::{EventMessage, PublishableEventMessage};

impl AxonMessage for GiftCardIssued {
    fn name() -> &'static str {
        "GiftCardIssued"
    }
}

impl AxonMessage for GiftCardRedeemed {
    fn name() -> &'static str {
        "GiftCardRedeemed"
    }
}

impl AxonMessage for GiftCardCanceled {
    fn name() -> &'static str {
        "GiftCardCanceled"
    }
}

#[derive(Debug)]
pub enum GiftCardEvent {
    Issue(GiftCardIssued),
    Redeem(GiftCardRedeemed),
    Cancel(GiftCardCanceled),
}

impl GiftCardEvent {
    pub fn get_name(&self) -> &'static str {
        match self {
            GiftCardEvent::Issue(_) => GiftCardIssued::name(),
            GiftCardEvent::Redeem(_) => GiftCardRedeemed::name(),
            GiftCardEvent::Cancel(_) => GiftCardCanceled::name(),
        }
    }
    pub fn get_payload(&self) -> Option<Option<Value>> {
        match self {
            GiftCardEvent::Issue(i) => message_to_payload(i),
            GiftCardEvent::Redeem(r) => message_to_payload(r),
            GiftCardEvent::Cancel(c) => message_to_payload(c),
        }
    }
}

pub fn to_publishable_event_message(
    name: &str,
    aggregate_id: Option<String>,
    aggregate_type: Option<String>,
    sequence_number: Option<i64>,
    payload: Option<Option<Value>>,
) -> PublishableEventMessage {
    PublishableEventMessage {
        payload_type: None,
        name: String::from(name),
        aggregate_id,
        aggregate_type,
        sequence_number,
        date_time: None,
        index: None,
        id: None,
        meta_data: None,
        payload,
        payload_revision: None,
    }
}

pub trait ContainsGiftCardEvent {
    fn get_gift_card_event(&self) -> Option<GiftCardEvent>;
}

impl ContainsGiftCardEvent for EventMessage {
    fn get_gift_card_event(&self) -> Option<GiftCardEvent> {
        let value = self.payload.clone().unwrap().unwrap();
        match self.name.as_str() {
            "GiftCardIssued" => {
                let gift_card_issued: GiftCardIssued = value_to_message(value);
                Some(GiftCardEvent::Issue(gift_card_issued))
            }
            "GiftCardRedeemed" => {
                let gift_card_redeemed: GiftCardRedeemed = value_to_message(value);
                Some(GiftCardEvent::Redeem(gift_card_redeemed))
            }
            "GiftCardCanceled" => {
                let gift_card_cancelled: GiftCardCanceled = value_to_message(value);
                Some(GiftCardEvent::Cancel(gift_card_cancelled))
            }
            _ => None,
        }
    }
}
