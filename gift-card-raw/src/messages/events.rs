use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use synapse_client::models::PublishableEventMessage;

#[derive(Serialize, Deserialize)]
pub struct GiftCardIssued {
    pub id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GiftCardRedeemed {
    pub id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
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

pub fn to_publishable_event_message<T>(
    name: &str,
    aggregate_id: Option<String>,
    sequence_number: Option<i64>,
    event: &T,
) -> PublishableEventMessage
where
    T: Serialize,
{
    PublishableEventMessage {
        payload_type: None,
        name: String::from(name),
        aggregate_id,
        aggregate_type: None,
        sequence_number,
        date_time: None,
        index: None,
        id: None,
        meta_data: None,
        payload: Some(Some(serde_json::to_value(event).unwrap())),
        payload_revision: None,
    }
}
