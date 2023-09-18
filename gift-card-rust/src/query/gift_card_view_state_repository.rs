use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

use async_trait::async_trait;
use derive_more::Display;
use fmodel_rust::materialized_view::ViewStateRepository;
use serde_derive::{Deserialize, Serialize};

use crate::gift_card_api::GiftCardEvent;
use crate::query::gift_card_event_handler::GiftCardViewState;

/// Error type for the application/aggregate
#[derive(Debug, Display, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum MaterializedViewError {
    FetchState(String),
    SaveState(String),
}

impl Error for MaterializedViewError {}

/// Struct to hold the view state in memory - not for production use, consider using a database
pub struct InMemoryViewStateRepository {
    pub(crate) states: Mutex<HashMap<String, GiftCardViewState>>,
}

impl InMemoryViewStateRepository {
    pub(crate) fn new() -> Self {
        InMemoryViewStateRepository {
            states: Mutex::new(HashMap::new()),
        }
    }
}

/// Implementation of [ViewStateRepository] for [InMemoryViewStateRepository]
#[async_trait]
impl ViewStateRepository<GiftCardEvent, GiftCardViewState> for InMemoryViewStateRepository {
    type Error = MaterializedViewError;

    async fn fetch_state(
        &self,
        event: &GiftCardEvent,
    ) -> Result<Option<GiftCardViewState>, MaterializedViewError> {
        Ok(self.states.lock().unwrap().get(&event.id()).cloned())
    }

    async fn save(
        &self,
        state: &GiftCardViewState,
    ) -> Result<GiftCardViewState, MaterializedViewError> {
        self.states
            .lock()
            .unwrap()
            .insert(state.id.to_owned(), state.clone());
        Ok(state.clone())
    }
}
