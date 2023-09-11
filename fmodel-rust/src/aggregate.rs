use std::fmt::{Debug, Display};

use async_trait::async_trait;

use crate::decider::{Decider, EventComputation, StateComputation};

/// Event repository trait
#[async_trait]
pub trait EventRepository<C, E> {
    /// The error type returned by the repository methods.
    type Error: std::error::Error + Send + Sync;

    /// Fetches events based on the command.
    async fn fetch_events(&self, command: &C) -> Result<Vec<E>, Self::Error>;

    /// Saves events.
    async fn save(&self, events: &Vec<E>) -> Result<Vec<E>, Self::Error>;
}

/// Event sourcing aggregate is using/delegating a `decider` of type [IDecider]<[C], [S], [E]>/ [EventComputation]<[C], [S], [E]> to handle commands and produce events.
pub struct EventSourcedAggregate<'a, C, S, E, R, Err>
    where
        R: EventRepository<C, E, Error=Err>,
{
    pub repository: R,
    pub decider: Decider<'a, C, S, E>,
}

impl<'a, C, S, E, R, Err> EventSourcedAggregate<'a, C, S, E, R, Err>
    where
        R: EventRepository<C, E, Error=Err>
{
    /// Handles the command by fetching the events from the repository, computing new events based on the current events and the command, and saving the new events to the repository.
    pub async fn handle(&self, command: &C) -> Result<Vec<E>, Err> {
        let events = self.repository.fetch_events(command).await?;
        let new_events = self.decider.compute_new_events(&events, command);
        let saved_events = self.repository.save(&new_events).await?;
        Ok(saved_events)
    }
}

/// State repository trait
pub trait StateRepository<C, S> {
    /// The error type returned by the repository methods.
    type Error: std::error::Error + Send + Sync;
    /// Fetches state based on the command.
    fn fetch_state(&self, command: &C) -> Result<Option<S>, Self::Error>;
    /// Saves state.
    fn save(&self, state: &S) -> Result<S, Self::Error>;
}

/// State stored aggregate is using/delegating a `decider` of type [Decider]<[C], [S], [E]> to handle commands and produce new state.
pub struct StateStoredAggregate<'a, C, S, E, R, Err>
    where
        R: StateRepository<C, S, Error=Err>,
{
    repository: R,
    decider: Decider<'a, C, S, E>,
}

impl<'a, C, S, E, R, Err> StateStoredAggregate<'a, C, S, E, R, Err>
    where
        R: StateRepository<C, S, Error=Err>,
{
    /// Handles the command by fetching the state from the repository, computing new state based on the current state and the command, and saving the new state to the repository.
    pub async fn handle(&self, command: &C) -> Result<S, Err> {
        let state = self.repository.fetch_state(command)?;
        let new_state = self.decider.compute_new_state(state, command);
        let saved_state = self.repository.save(&new_state)?;
        Ok(saved_state)
    }
}
