use std::error::Error;

use async_trait::async_trait;
use derive_more::Display;
use fmodel_rust::aggregate::EventRepository;
use serde_derive::{Deserialize, Serialize};
use synapse_client::apis::aggregate_api::read_aggregate_events;
use synapse_client::apis::configuration;
use synapse_client::apis::events_api::publish_event_message;
use synapse_client::models::{EventMessage, PublishableEventMessage};

use crate::gift_card_api::{GiftCardCommand, GiftCardEvent};

/// Error type for the application/aggregate
#[derive(Debug, Display, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum AggregateError {
    FetchEvents(String),
    SaveEvents(String),
    FetchState(String),
    SaveState(String),
}

impl Error for AggregateError {}

/// Map to domain events of type GiftCardEvent
trait ToGiftCardEvent {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent>;
}

/// Map from Axon EventMessage to domain events of type GiftCardEvent
impl ToGiftCardEvent for EventMessage {
    fn to_gift_card_event(&self) -> Option<GiftCardEvent> {
        let value = self.payload.clone().unwrap().unwrap();
        let event = serde_json::from_value(value);
        match event {
            Ok(event) => Some(event),
            Err(_err) => None,
        }
    }
}

/// Map to Axon EventMessage
trait ToEventMessage {
    fn to_event_message(&self, version: i64) -> PublishableEventMessage;
}

/// Map from domain events of type GiftCardEvent to Axon EventMessage
impl ToEventMessage for GiftCardEvent {
    fn to_event_message(&self, version: i64) -> PublishableEventMessage {
        let payload = serde_json::to_value(self).unwrap();
        PublishableEventMessage {
            payload_type: Some(self.payload_type()),
            name: self.payload_type(),
            aggregate_id: Some(self.id()),
            aggregate_type: Some(self.aggregate_type()),
            sequence_number: Some(version),
            date_time: None,
            index: None,
            id: None,
            meta_data: None,
            payload: Some(Some(payload)),
            payload_revision: None,
        }
    }
}

/// Axon Server event repository
pub struct AxonServerEventRepository {
    pub configuration: configuration::Configuration,
    pub context: String,
}

/// Event repository implementation for Axon Server
#[async_trait]
impl EventRepository<GiftCardCommand, GiftCardEvent> for AxonServerEventRepository {
    type Error = AggregateError;
    type Version = i64;

    async fn fetch_events(
        &self,
        command: &GiftCardCommand,
    ) -> Result<Vec<(GiftCardEvent, i64)>, AggregateError> {
        let result = read_aggregate_events(&self.configuration, &self.context, &command.id()).await;
        match result {
            Ok(events) => Ok(events
                .items
                .unwrap_or_default()
                .into_iter()
                .map(|event| {
                    (
                        event.to_gift_card_event().unwrap(),
                        event.sequence_number.unwrap(),
                    )
                })
                .collect()),
            Err(err) => Err(AggregateError::FetchEvents(err.to_string())),
        }
    }

    async fn save(
        &self,
        events: &[GiftCardEvent],
        version: &Option<i64>,
    ) -> Result<Vec<(GiftCardEvent, i64)>, AggregateError> {
        let mut saved_events: Vec<(GiftCardEvent, i64)> = vec![];
        let mut version = version.unwrap_or(-1);
        for evt in events {
            version += 1;
            let result = publish_event_message(
                &self.configuration,
                &self.context,
                Some(evt.to_event_message(version)),
            )
                .await;
            match result {
                Ok(_) => saved_events.push((evt.to_owned(), version)),
                Err(err) => return Err(AggregateError::SaveEvents(err.to_string())),
            };
        }
        Ok(saved_events)
    }
}
