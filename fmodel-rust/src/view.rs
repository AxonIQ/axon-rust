use crate::{EvolveFunction, InitialStateFunction};

/// [View] represents the event handling algorithm, responsible for translating the events into denormalized state, which is more adequate for querying.
/// It has two generic parameters `S`/State, `E`/Event , representing the type of the values that View may contain or use.
/// In this struct definition:
///
///  -'a is used as a lifetime parameter, indicating that all references contained within the struct (e.g., references within the function closures) must have a lifetime that is at least as long as 'a. This is a common approach for ensuring that the references within the struct are valid for the entire lifetime of the View struct.
///  - The function closures (e.g. evolve, initial_state) are defined to accept references with the same lifetime 'a, which ensures that they can reference the same data without any lifetime issues.
pub struct View<'a, S: 'a, E: 'a> {
    /// The `evolve` function is the main state evolution algorithm.
    pub evolve: EvolveFunction<'a, S, E>,
    /// The `initial_state` function is the initial state.
    pub initial_state: InitialStateFunction<'a, S>,
}

impl<'a, S, E> View<'a, S, E> {
    /// Creates a new instance of [View]`<S2, E>` by mapping the `evolve` function to a new function that takes a `&S2` and a `&E` and returns a `S`, and mapping the `initial_state` function to a new function that returns a `S2`.
    pub fn map_state<S2, F1, F2>(self, f1: &'a F1, f2: &'a F2) -> View<'a, S2, E>
        where
            F1: Fn(&S2) -> S  + Send + Sync,
            F2: Fn(&S) -> S2 + Send + Sync,
    {
        let new_evolve = Box::new(move |s2: &S2, e: &E| {
            let s = f1(s2);
            f2(&(self.evolve)(&s, e))
        });

        let new_initial_state = Box::new(move || {
            f2(&(self.initial_state)())
        });

        View {
            evolve: new_evolve,
            initial_state: new_initial_state,
        }
    }

    /// Creates a new instance of [View]`<S, E2>` by mapping the `evolve` function to a new function that takes a `&S` and a `&E2` and returns a `S`, and mapping the `initial_state` function to a new function that returns a `S`.
    pub fn map_event<E2, F>(self, f: &'a F) -> View<'a, S, E2>
        where
            F: Fn(&E2) -> E + Send + Sync,
    {
        let new_evolve = Box::new(move |s: &S, e2: &E2| {
            let e = f(e2);
            (self.evolve)(s, &e)
        });

        let new_initial_state = Box::new(move || {
            (self.initial_state)()
        });

        View {
            evolve: new_evolve,
            initial_state: new_initial_state,
        }
    }
}

pub trait ViewStateComputation<E, S> {
    /// Computes new state based on the current state and the events.
    fn compute_new_state(&self, current_state: Option<S>, events: &[&E]) -> S;
}

impl<'a, S, E> ViewStateComputation<E, S> for View<'a, S, E> {
    /// Computes new state based on the current state and the events.
    fn compute_new_state(&self, current_state: Option<S>, events: &[&E]) -> S {
        let effective_current_state = current_state.unwrap_or_else(|| (self.initial_state)());
        events.iter().fold(effective_current_state, |state, event| {
            (self.evolve)(&state, event)
        })
    }
}