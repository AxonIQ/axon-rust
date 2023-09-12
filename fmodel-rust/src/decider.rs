use crate::{DecideFunction, EvolveFunction, InitialStateFunction};

/// [Decider] represents the main decision-making algorithm.
/// It has three generic parameters `C`/`Command`, `S`/`State`, `E`/`Event` , representing the type of the values that Decider may contain or use.
/// In this struct definition:
///
///  -'a is used as a lifetime parameter, indicating that all references contained within the struct (e.g., references within the function closures) must have a lifetime that is at least as long as 'a. This is a common approach for ensuring that the references within the struct are valid for the entire lifetime of the Decider struct.
///  - The function closures (e.g., decide, evolve, initial_state) are defined to accept references with the same lifetime 'a, which ensures that they can reference the same data without any lifetime issues.
pub struct Decider<'a, C: 'a, S: 'a, E: 'a> {
    /// The `decide` function is used to decide which events to produce based on the command and the current state.
    pub decide: DecideFunction<'a, C, S, E>,
    /// The `evolve` function is used to evolve the state based on the current state and the event.
    pub evolve: EvolveFunction<'a, S, E>,
    /// The `initial_state` function is used to produce the initial state of the decider.
    pub initial_state: InitialStateFunction<'a, S>,
}

impl<'a, C, S, E> Decider<'a, C, S, E> {
    /// Creates a new instance of [Decider]`<C, S2, E>` by mapping the `decide` function to a new function that takes a `&C` and a `&S2` and returns a `Vec<E>`, and mapping the `evolve` function to a new function that takes a `&S2` and a `&E` and returns a `S`, and mapping the `initial_state` function to a new function that returns a `S2`.
    pub fn map_state<S2, F1, F2>(self, f1: &'a F1, f2: &'a F2) -> Decider<'a, C, S2, E>
        where
            F1: Fn(&S2) -> S,
            F2: Fn(&S) -> S2,
    {
        let new_decide = Box::pin(move |c: &C, s2: &S2| {
            let s = f1(s2);
            (self.decide)(c, &s)
        });

        let new_evolve = Box::pin(move |s2: &S2, e: &E| {
            let s = f1(s2);
            f2(&(self.evolve)(&s, e))
        });

        let new_initial_state = Box::pin(move || {
            f2(&(self.initial_state)())
        });

        Decider {
            decide: new_decide,
            evolve: new_evolve,
            initial_state: new_initial_state,
        }
    }

    /// Creates a new instance of [Decider]`<C, S, E2>` by mapping the `decide` function to a new function that takes a `&C` and a `&S` and returns a `Vec<E2>`, and mapping the `evolve` function to a new function that takes a `&S` and a `&E2` and returns a `S`, and mapping the `initial_state` function to a new function that returns a `S`.
    pub fn map_event<E2, F1, F2>(self, f1: &'a F1, f2: &'a F2) -> Decider<'a, C, S, E2>
        where
            F1: Fn(&E2) -> E,
            F2: Fn(&E) -> E2,
    {
        let new_decide = Box::pin(move |c: &C, s: &S| {
            (self.decide)(c, s).into_iter().map(|e: E| { f2(&e) }).collect()
        });

        let new_evolve = Box::pin(move |s: &S, e2: &E2| {
            let e = f1(e2);
            (self.evolve)(s, &e)
        });

        let new_initial_state = Box::pin(move || {
            (self.initial_state)()
        });

        Decider {
            decide: new_decide,
            evolve: new_evolve,
            initial_state: new_initial_state,
        }
    }

    /// Creates a new instance of [Decider]`<C2, S, E>` by mapping the `decide` function to a new function that takes a `&C2` and a `&S` and returns a `Vec<E>`, and mapping the `evolve` function to a new function that takes a `&S` and a `&E` and returns a `S`, and mapping the `initial_state` function to a new function that returns a `S`.
    pub fn map_command<C2, F>(self, f: &'a F) -> Decider<'a, C2, S, E>
        where
            F: Fn(&C2) -> C,
    {
        let new_decide = Box::pin(move |c2: &C2, s: &S| {
            let c = f(c2);
            (self.decide)(&c, s)
        });

        let new_evolve = Box::pin(move |s: &S, e: &E| {
            (self.evolve)(s, e)
        });

        let new_initial_state = Box::pin(move || {
            (self.initial_state)()
        });

        Decider {
            decide: new_decide,
            evolve: new_evolve,
            initial_state: new_initial_state,
        }
    }
}


/// Formalizes the `Event Computation` algorithm / event sourced system for the `decider` to handle commands based on the current events, and produce new events.
pub trait EventComputation<C, E> {
    /// Computes new events based on the current events and the command.
    fn compute_new_events(&self, current_events: &[E], command: &C) -> Vec<E>;
}


/// Formalizes the `State Computation` algorithm / state-stored system for the `decider` to handle commands based on the current state, and produce new state.
pub trait StateComputation<C, S> {
    /// Computes new state based on the current state and the command.
    fn compute_new_state(&self, current_state: Option<S>, command: &C) -> S;
}

impl<'a, C, S, E> EventComputation<C, E> for Decider<'a, C, S, E> {
    /// Computes new events based on the current events and the command.
    fn compute_new_events(&self, current_events: &[E], command: &C) -> Vec<E> {
        let current_state: S = current_events.iter().fold((self.initial_state)(), |state, event| {
            (self.evolve)(&state, event)
        });
        (self.decide)(command, &current_state)
    }
}

impl<'a, C, S, E> StateComputation<C, S> for Decider<'a, C, S, E> {
    /// Computes new state based on the current state and the command.
    fn compute_new_state(&self, current_state: Option<S>, command: &C) -> S {
        let effective_current_state = current_state.unwrap_or_else(|| (self.initial_state)());
        let events = (self.decide)(command, &effective_current_state);
        events.into_iter().fold(effective_current_state, |state, event| {
            (self.evolve)(&state, &event)
        })
    }
}