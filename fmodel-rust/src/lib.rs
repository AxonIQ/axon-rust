use std::pin::Pin;

pub mod aggregate;
pub mod decider;
pub mod materialized_view;
pub mod view;

/// The [DecideFunction] function is used to decide which events to produce based on the command and the current state.
pub type DecideFunction<'a, C, S, E> = Pin<Box<dyn Fn(&C, &S) -> Vec<E> + 'a>>;
/// The [EvolveFunction] function is used to evolve the state based on the current state and the event.
pub type EvolveFunction<'a, S, E> = Pin<Box<dyn Fn(&S, &E) -> S + 'a>>;
/// The [InitialStateFunction] function is used to produce the initial state.
pub type InitialStateFunction<'a, S> = Pin<Box<dyn Fn() -> S + 'a>>;

