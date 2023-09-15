pub mod aggregate;
pub mod decider;
pub mod materialized_view;
pub mod view;

/// The [DecideFunction] function is used to decide which events to produce based on the command and the current state.
pub type DecideFunction<'a, C, S, E> = Box<dyn Fn(&C, &S) -> Vec<E> + 'a + Send + Sync>;
/// The [EvolveFunction] function is used to evolve the state based on the current state and the event.
pub type EvolveFunction<'a, S, E> = Box<dyn Fn(&S, &E) -> S + 'a + Send + Sync>;
/// The [InitialStateFunction] function is used to produce the initial state.
pub type InitialStateFunction<'a, S> = Box<dyn Fn() -> S + 'a + Send + Sync>;

