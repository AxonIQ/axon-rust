use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use synapse_client::models::{EventMessage, PublishableEventMessage};

#[derive(Serialize, Deserialize, Debug)]
pub struct GiftCardIssued {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiftCardRedeemed {
    pub id: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GiftCardCanceled {
    pub id: String,
}

pub trait AxonDomainEvent<'a> {
    fn body(&self) -> Option<PathBuf>;
    fn event_name(&'a self) -> &'a str;
    fn axon_iq_payload_revision(&'a self) -> Option<&'a str>;
    fn axon_iq_aggregate_id(&'a self) -> Option<&'a str>;
    fn axon_iq_aggregate_type(&'a self) -> Option<&'a str>;
    fn axon_iq_sequence_number(&self) -> Option<i64>;
    fn axon_iq_data_time(&self) -> Option<String>;
}

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
    pub fn get_payload(&self) -> Value {
        match self {
            GiftCardEvent::Issue(i) => serde_json::to_value(i).unwrap(),
            GiftCardEvent::Redeem(r) => serde_json::to_value(r).unwrap(),
            GiftCardEvent::Cancel(c) => serde_json::to_value(c).unwrap(),
        }
    }
}

pub fn to_publishable_event_message(
    name: &str,
    aggregate_id: Option<String>,
    aggregate_type: Option<String>,
    sequence_number: Option<i64>,
    event: Value,
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
        payload: Some(Some(event)),
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
                let gift_card_issued: GiftCardIssued = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Issue(gift_card_issued))
            }
            "GiftCardRedeemed" => {
                let gift_card_redeemed: GiftCardRedeemed = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Redeem(gift_card_redeemed))
            }
            "GiftCardCancelled" => {
                let gift_card_cancelled: GiftCardCanceled = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Cancel(gift_card_cancelled))
            }
            _ => None,
        }
    }
}
