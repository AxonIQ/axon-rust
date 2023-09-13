use std::error::Error;

use async_trait::async_trait;
use derive_more::Display;
use synapse_client::apis::aggregate_api::read_aggregate_events;
use synapse_client::apis::configuration;
use synapse_client::apis::events_api::publish_event_message;
use synapse_client::models::{EventMessage, PublishableEventMessage};

use fmodel_rust::aggregate::EventRepository;

use crate::api::{GiftCardCanceled, GiftCardCommand, GiftCardEvent, GiftCardIssued, GiftCardRedeemed};

/// Error type for the application/aggregate
#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum AggregateError {
    FetchEvents(String),
    SaveEvents(String),
    FetchState(String),
    SaveState(String),
}

impl Error for AggregateError {}

pub trait ToGiftCardEvent {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent>;
}

impl ToGiftCardEvent for EventMessage {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent> {
        let value = self.payload.clone().unwrap().unwrap();
        match self.name.as_str() {
            "GiftCardIssued" => {
                let issue_gift_card: GiftCardIssued = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Issue(issue_gift_card))
            }
            "GiftCardRedeemed" => {
                let redeem_gift_card: GiftCardRedeemed = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Redeem(redeem_gift_card))
            }
            "GiftCardCanceled" => {
                let cancel_gift_card: GiftCardCanceled = serde_json::from_value(value).unwrap();
                Some(GiftCardEvent::Cancel(cancel_gift_card))
            }
            _ => None,
        }
    }
}

pub trait ToEventMessage {
    fn to_event_message(&self) -> PublishableEventMessage;
}

impl ToEventMessage for GiftCardEvent {
    fn to_event_message(&self) -> PublishableEventMessage {
        match self {
            GiftCardEvent::Issue(evt) => {
                let event = serde_json::to_value(evt).unwrap();
                PublishableEventMessage {
                    payload_type: Some("GiftCardIssued".to_string()),
                    name: "GiftCardIssued".to_string(),
                    aggregate_id: Some(evt.id.to_owned()),
                    aggregate_type: Some("GiftCard".to_string()),
                    sequence_number: None, // TODO: how to get this?
                    date_time: None,
                    index: None,
                    id: None,
                    meta_data: None,
                    payload: Some(Some(event)),
                    payload_revision: None,
                }
            }
            GiftCardEvent::Redeem(evt) => {
                let event = serde_json::to_value(evt).unwrap();
                PublishableEventMessage {
                    payload_type: Some("GiftCardRedeemed".to_string()),
                    name: "GiftCardRedeemed".to_string(),
                    aggregate_id: Some(evt.id.to_owned()),
                    aggregate_type: Some("GiftCard".to_string()),
                    sequence_number: None, // TODO: how to get this?
                    date_time: None,
                    index: None,
                    id: None,
                    meta_data: None,
                    payload: Some(Some(event)),
                    payload_revision: None,
                }
            }
            GiftCardEvent::Cancel(evt) => {
                let event = serde_json::to_value(evt).unwrap();
                PublishableEventMessage {
                    payload_type: Some("GiftCardCanceled".to_string()),
                    name: "GiftCardCanceled".to_string(),
                    aggregate_id: Some(evt.id.to_owned()),
                    aggregate_type: Some("GiftCard".to_string()),
                    sequence_number: None, // TODO: how to get this?
                    date_time: None,
                    index: None,
                    id: None,
                    meta_data: None,
                    payload: Some(Some(event)),
                    payload_revision: None,
                }
            }
        }
    }
}

pub struct AxonServerEventRepository {
    pub configuration: configuration::Configuration,
    pub context: String,
}

#[async_trait]
impl EventRepository<GiftCardCommand, GiftCardEvent> for AxonServerEventRepository {
    type Error = AggregateError;
    type Version = i64;

    async fn fetch_events(&self, command: &GiftCardCommand) -> Result<Vec<(GiftCardEvent, i64)>, AggregateError> {
        let result = read_aggregate_events(&self.configuration, &self.context, &command.id()).await;
        match result {
            Ok(events) => {
                Ok(events
                    .items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|event| (event.to_gift_card_event().unwrap(), event.sequence_number.unwrap()))
                    .collect())
            }
            Err(err) => Err(AggregateError::FetchEvents(err.to_string())),
        }
    }

    async fn save(&self, events: &[GiftCardEvent], version: &Option<i64>) -> Result<Vec<(GiftCardEvent, i64)>, AggregateError> {
        let mut saved_events: Vec<(GiftCardEvent, i64)> = vec![];
        let mut version = version.unwrap_or(-1);
        for evt in events {
            let result = publish_event_message(&self.configuration, &self.context, Some(evt.to_event_message())).await;
            match result {
                Ok(_) => {
                    version += 1;
                    saved_events.push((evt.to_owned(), version))
                }
                Err(err) => return Err(AggregateError::SaveEvents(err.to_string())),
            };
        }
        Ok(saved_events)
    }
}
