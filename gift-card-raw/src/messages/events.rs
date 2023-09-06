use crate::messages::AxonMessage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

pub struct DomainEvent<'a> {
    body: String,
    name: &'a str,
    aggregate_id: String,
    sequence_number: i64,
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

impl<'a> AxonDomainEvent<'a> for DomainEvent<'a> {
    fn body(&self) -> Option<PathBuf> {
        Some(self.body.parse().unwrap())
    }

    fn event_name(&self) -> &'_ str {
        self.name
    }

    fn axon_iq_payload_revision(&self) -> Option<&'_ str> {
        None
    }

    fn axon_iq_aggregate_id(&self) -> Option<&'_ str> {
        Some(&*self.aggregate_id)
    }

    fn axon_iq_aggregate_type(&self) -> Option<&'_ str> {
        None
    }

    fn axon_iq_sequence_number(&self) -> Option<i64> {
        Some(self.sequence_number)
    }

    fn axon_iq_data_time(&self) -> Option<String> {
        None
    }
}
